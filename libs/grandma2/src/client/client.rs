use std::{collections::HashMap, time::Duration};

extern crate md5;

use futures_util::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use tokio::{
    net::TcpStream,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    time::{interval, Interval},
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

use crate::{client::ReceiveMsg, types::{ButtonRange, FaderRange}};
use crate::{
    interface::{MaEvent, MaRequest},
    types::{ButtonData, FaderData, Ma2Data},
    ButtonExecutor, FaderExecutor,
};
use crate::{Ma2Error, Result};

use super::{messages::{Request, Response}, SendMsg};

const MAX_REQUESTS: u8 = 9;

#[derive(Debug)]
pub struct GrandMa2Client {
    // internals
    ws_sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    ws_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    rx_request: UnboundedReceiver<MaRequest>,
    tx_event: UnboundedSender<MaEvent>,
    interval: Interval,

    // config
    username: String,
    password: String,
    subscribed_button_range: Option<ButtonRange>,
    subscribed_fader_range: Option<FaderRange>,

    // state
    state: Ma2State,
    logged_in: bool,
    session_id: i8,
    num_requests: u8,
}

impl GrandMa2Client {
    /// Creates a new GrandMa2 Client
    pub fn new(
        ws_sink: WebSocketStream<MaybeTlsStream<TcpStream>>,
        rx_request: UnboundedReceiver<MaRequest>,
        tx_event: UnboundedSender<MaEvent>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        let (ws_sink, ws_stream) = ws_sink.split();
        Self {
            // internals
            ws_sink,
            ws_stream,
            rx_request,
            tx_event,
            interval: interval(Duration::from_millis(100)),

            // config
            username: username.into(),
            password: password.into(),
            subscribed_button_range: None,
            subscribed_fader_range: None,

            // state
            logged_in: false,
            session_id: -1,
            num_requests: 0,
            state: Ma2State::new(),
        }
    }

    /// This method is running the main loop for the GrandMa2 client
    ///
    /// `run` has be executed right after the connection has been established.
    pub async fn run(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                msg = self.rx_request.recv() => {
                    match msg {
                        Some(msg) => {
                            let close = self.handle_request(msg).await?;

                            // Execute if a close message has been handled
                            if close {
                                return Ok(())
                            }
                        }
                        None => {return Err(Ma2Error::RequestChannelClosed.into());}
                    }
                }
                Some(Ok(msg)) = self.ws_stream.next() => {
                    // println!("[GrandMa2] Receiving RAW {msg:?}");
                    match msg {
                        Message::Text(msg_string) => {
                            let msg: ReceiveMsg = serde_json::from_value(serde_json::from_str(&msg_string)
                                .unwrap()).map_err(|err| Ma2Error::CouldNotDeserializeReceiveMsg(msg_string, Box::new(err)))?;
                            println!("[GrandMa2] Receive: {:?}", msg);
                            self.handle_ma2_message(msg).await?;
                        }
                        Message::Ping(data) => {
                            // Does this still work without
                            //self.send_ma2_raw(Message::Pong(data)).await?;
                        }
                        _ => {todo!("Cannot handle this")}
                    }
                },
                _ = self.interval.tick() => {
                    self.on_interval().await?;
                }
            }
        }
    }

    async fn handle_ma2_message(&mut self, msg: ReceiveMsg) -> Result<()> {
        // Handle weird GrandMa behaviour
        self.num_requests += 1;
        if self.num_requests >= MAX_REQUESTS {
            self.send_session().await?;
            self.num_requests = 0;
        }

        match msg {
            ReceiveMsg::Session {
                force_login: Some(true),
                session,
                ..
            } => {
                self.session_id = session;
                self.send_login().await?;
                Ok(())
            }
            ReceiveMsg::Session { session, .. } => match session {
                ..=-1 => Err(Ma2Error::WebRemoteDisabled.into()),
                0 => {
                    self.send_session().await?;
                    Err(Ma2Error::ConnectedButInvalidSessionId.into())
                }
                _ => {
                    self.session_id = session;
                    Ok(())
                }
            },
            ReceiveMsg::Status { status, app_type }
                if status.as_ref() == "server ready" && app_type.as_ref() == "gma2" =>
            {
                self.send_ma2_msg(SendMsg::Session { session: 0 }).await?;
                Ok(())
            }
            ReceiveMsg::Response(Response::Login { result, .. }) => {
                self.send_interface(MaEvent::LoginSuccessful(result))?;

                if result {
                    self.logged_in = true;
                    Ok(())
                } else {
                    Err(Ma2Error::LoginFailed {
                        username: self.username.clone(),
                        password: self.password.clone(),
                        hashed_password: self.get_hashed_password(),
                    }
                    .into())
                }
            }
            ReceiveMsg::Response(Response::Playbacks { item_groups, .. }) => {
                let diff = self.state.diff_and_update(item_groups);
                for channel in diff.fader_data {
                    let msg = MaEvent::FaderChanged(channel);
                    self.send_interface(msg)?;
                }
                for channel in diff.button_data {
                    let msg = MaEvent::ButtonChanged(channel);
                    self.send_interface(msg)?;
                }
                Ok(())
            }
            _ => Err(Ma2Error::MessageHandlerNotImplemented(msg).into()),
        }
    }

    async fn handle_request(&mut self, msg: MaRequest) -> Result<bool> {
        match msg {
            // Disconnect
            MaRequest::Disconnect => {
                self.close_connection().await;
                return Ok(true);
            }
            MaRequest::SubscribeButton(start, end) => {
                self.subscribed_button_range = Some(ButtonRange::from(start, end)?);
            }
            MaRequest::SubscribeFader(start, end) => {
                self.subscribed_fader_range = Some(FaderRange::from(start, end)?);
            }
            _ => {
                todo!("Cannot handle message {:?}", msg)
            }
        };
        Ok(false)
    }

    async fn on_interval(&mut self) -> Result<()> {
        if let Some(subscribed_button_range) = self.subscribed_button_range {
            let msg = SendMsg::Request(Request::Playbacks {
                start_index: vec![subscribed_button_range.start_button.id().into()],
                items_count: vec![subscribed_button_range.num_buttons.into()],
                page_index: 0,
                items_type: vec![3],
                view: 2,
                exec_button_view_mode: 1,
                buttons_view_mode: 0,
                max_requests: 1,
                session: self.session_id,
            });
            self.send_ma2_msg(msg).await?
        }
        if let Some(subscribed_fader_range) = self.subscribed_fader_range {
            let msg = SendMsg::Request(Request::Playbacks {
                start_index: vec![subscribed_fader_range.start_fader.id().into()],
                items_count: vec![subscribed_fader_range.num_faders.into()],
                page_index: 0,
                items_type: vec![2],
                view: 2,
                exec_button_view_mode: 1,
                buttons_view_mode: 0,
                max_requests: 1,
                session: self.session_id,
            });
            self.send_ma2_msg(msg).await?
        }
        Ok(())
    }

    async fn send_ma2_msg(&mut self, msg: SendMsg) -> Result<()> {
        println!("[GrandMa2] Sending: {:?}", msg);
        let msg_string =
            serde_json::to_string(&msg).or(Err(Ma2Error::SerializeError(msg.clone())))?;
        self.send_ma2_raw(Message::Text(msg_string)).await?;
        Ok(())
    }

    async fn send_ma2_raw(&mut self, msg: Message) -> Result<()> {
        self.ws_sink
            .send(msg)
            .await
            .map_err(|e| Ma2Error::WebsocketFailedToSend(e).into())
    }

    fn send_interface(&self, message: MaEvent) -> Result<()> {
        self.tx_event
            .send(message)
            .map_err(|_| Ma2Error::EventChannelClosed.into())
    }

    async fn send_session(&mut self) -> Result<()> {
        let session_msg = SendMsg::Session {
            session: self.session_id,
        };
        self.send_ma2_msg(session_msg).await
    }

    async fn send_login(&mut self) -> Result<()> {
        let login_msg = SendMsg::Request(Request::Login {
            username: self.username.clone(),
            password: self.get_hashed_password(),
            max_requests: 10,
            session: self.session_id,
        });
        self.send_ma2_msg(login_msg).await
    }

    async fn close_connection(&mut self) {
        let close_msg = SendMsg::Request(Request::Close {
            session: self.session_id,
            max_requests: 10,
        });

        // Try to send logout and close the websocket
        // if it fails just accept it
        self.send_ma2_msg(close_msg).await.unwrap_or(());
        self.ws_sink.close().await.unwrap_or(());
    }

    fn get_hashed_password(&self) -> String {
        format!("{:x}", md5::compute(&self.password))
    }
}

#[derive(Debug)]
pub struct Ma2State {
    buttons: HashMap<ButtonExecutor, ButtonData>,
    faders: HashMap<FaderExecutor, FaderData>,
}

impl Ma2State {
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),
            faders: HashMap::new(),
        }
    }

    pub fn get_button(&self, button: ButtonExecutor) -> Option<&ButtonData> {
        self.buttons.get(&button)
    }

    pub fn get_fader(&self, fader: FaderExecutor) -> Option<&FaderData> {
        self.faders.get(&fader)
    }

    pub fn diff_and_update(&mut self, data: Ma2Data) -> Ma2Data {
        let Ma2Data {
            fader_data,
            button_data,
        } = data;

        let mut fader_changes = Vec::new();
        for fader_data in fader_data.into_iter() {
            if let Some(old_fader_data) = self.faders.get_mut(fader_data.get_executer()) {
                if old_fader_data != &fader_data {
                    fader_changes.push(fader_data.clone());
                    *old_fader_data = fader_data;
                }
            } else {
                self.faders
                    .insert(fader_data.get_executer().to_owned(), fader_data.clone());
                fader_changes.push(fader_data);
            }
        }
        let mut button_changes = Vec::new();
        for button_data in button_data.into_iter() {
            if let Some(old_button_data) = self.buttons.get_mut(button_data.get_executer()) {
                if old_button_data != &button_data {
                    button_changes.push(button_data.clone());
                    *old_button_data = button_data;
                }
            } else {
                self.buttons
                    .insert(button_data.get_executer().to_owned(), button_data.clone());
                button_changes.push(button_data);
            }
        }
        Ma2Data::new(fader_changes, button_changes)
    }
}
