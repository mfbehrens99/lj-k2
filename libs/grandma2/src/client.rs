use std::time::Duration;

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

use crate::interface_msg::{MaEvent, MaRequest};
use crate::{
    interface_msg::{ButtonRange, FaderRange},
    ma2_msg::{ReceiveMsg, Request, Response, SendMsg},
};
use crate::{Ma2Error, Result};

const MAX_REQUESTS: u8 = 9;

#[derive(Debug)]
pub(crate) struct GrandMa2Client {
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

            // states
            logged_in: false,
            session_id: -1,
            num_requests: 0,
        }
    }
    /// The main loop of the GrandMa2 client
    ///
    /// The main purpose of this function is to handle the error
    /// of the run_error method.
    pub async fn run(&mut self) {
        // Handle the error apropriately
        match self.run_error().await {
            Ok(_) => {
                println!("[GrandMa2 Client] Exited Gracefully");
                self.send_interface(MaEvent::Disconnected);
            }
            Err(err) => {
                println!("[GrandMa2 Client] Exited with error {:?}", err);
                self.send_interface(MaEvent::Error(err));
            }
        }
    }

    /// This method is running the actual main loop for the GrandMa2 client
    ///
    /// It is executed by the run method to handle the error gracefully
    async fn run_error(&mut self) -> Result<()> {
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
                        None => {return Err(Ma2Error::RequestChannelClosed);}
                    }
                }
                Some(Ok(msg)) = self.ws_stream.next() => {
                    // println!("[GrandMa2] Receiving RAW {msg:?}");
                    match msg {
                        Message::Text(msg_string) => {
                            let msg: ReceiveMsg = serde_json::from_str(&msg_string)
                                .expect(&format!("Could not parse message: '{}'", msg_string));
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
                ..=-1 => Err(Ma2Error::WebRemoteDisabled),
                0 => {
                    self.send_session().await?;
                    Err(Ma2Error::ConnectedButInvalidSessionId)
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
                self.send_interface(MaEvent::LoginSuccessful(result));

                if result {
                    self.logged_in = true;
                    Ok(())
                } else {
                    Err(Ma2Error::LoginFailed {
                        username: self.username.clone(),
                        password: self.password.clone(),
                        hashed_password: self.get_hashed_password(),
                    })
                }
            }
            _ => Err(Ma2Error::MessageHandlerNotImplemented(msg)),
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
            .map_err(|e| Ma2Error::FailedToSend(e))
    }

    fn send_interface(&self, message: MaEvent) {
        self.tx_event.send(message).unwrap();
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
