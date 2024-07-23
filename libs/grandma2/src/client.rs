extern crate md5;

use futures_util::{
    stream::{SplitSink, StreamExt},
    SinkExt,
};
use tokio::{
    net::TcpStream,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

use crate::messages::{ReceiveMsg, Request, Response, SendMsg};

const MAX_REQUESTS: u8 = 9;

#[derive(Debug)]
enum Error {
    MessageHandlerNotImplemented,
    GrandMA2Connection,
    WebRemoteDisabled,
    LoginFailed,
}

#[derive(Debug)]
pub enum MaRequest {
    Disconnect,
    Subscribe,
    SetFader,
    PressButton,
}

#[derive(Debug)]
pub enum MaEvent {
    Disconnect,
    FaderChanged,
    ButtonChanged,
}

#[derive(Debug)]
pub struct GrandMa2Client {
    url: String,
    username: String,
    password: String,
    ws_sink: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    rx_request: UnboundedReceiver<MaRequest>,
    tx_event: UnboundedSender<MaEvent>,
    logged_in: bool,
    session_id: i8,
    num_requests: u8,
}

impl GrandMa2Client {
    pub fn new<T, U>(
        url: T,
        username: T,
        password: U,
        rx_request: UnboundedReceiver<MaRequest>,
        tx_event: UnboundedSender<MaEvent>,
    ) -> Self
    where
        T: Into<String>,
        U: AsRef<[u8]>,
    {
        Self {
            url: url.into(),
            username: username.into(),
            password: format!("{:x}", md5::compute(password)),
            ws_sink: None,
            rx_request,
            tx_event,
            logged_in: false,
            session_id: -1,
            num_requests: 0,
        }
    }

    pub async fn run(&mut self) {
        let (ws_stream, _response) = tokio_tungstenite::connect_async(self.url.to_string())
            .await
            .expect("Failed to connect");

        let (ws_sink, mut ws_socket) = ws_stream.split();
        self.ws_sink = Some(ws_sink);

        loop {
            tokio::select! {
                Some(msg) = self.rx_request.recv() => {
                    match msg {
                        MaRequest::Disconnect => {},
                        _ => {todo!()},
                    };
                }
                Some(Ok(msg)) = ws_socket.next() => {
                    // println!("[GrandMa2] Receiving RAW {msg:?}");
                    match msg {
                        Message::Text(msg_string) => {
                            let msg: ReceiveMsg = serde_json::from_str(&msg_string)
                                .expect(&format!("Could not parse message: '{}'", msg_string));
                            println!("[GrandMa2] Receive: {:?}", msg);
                            self.handle_message(msg).await.unwrap();
                        }
                        Message::Ping(data) => {
                            let _ = self.ws_sink.as_mut().expect("").send(Message::Pong(data)).await;
                        }
                        _ => {todo!("Cannot handle this")}
                    }
                },
            }
        }
    }

    async fn handle_message(&mut self, msg: ReceiveMsg) -> Result<(), Error> {
        // Handle weird GrandMa behaviour
        self.num_requests += 1;
        if self.num_requests >= MAX_REQUESTS {
            self.send_session().await;
            self.num_requests = 0;
        }

        match msg {
            ReceiveMsg::Session {force_login: Some(true), session, ..} => {
                self.session_id = session;
                self.send_login().await;
                Ok(())
            }
            ReceiveMsg::Session { session, .. } => match session {
                ..=-1 => Err(Error::WebRemoteDisabled),
                0 => {
                    self.send_session().await;
                    Err(Error::GrandMA2Connection)
                }
                _ => {
                    self.session_id = session;
                    Ok(())
                }
            },
            ReceiveMsg::Status { status, app_type }
                if status.as_ref() == "server ready" && app_type.as_ref() == "gma2" =>
            {
                self.send(SendMsg::Session { session: 0 }).await;
                Ok(())
            }
            ReceiveMsg::Response(Response::Login { result: true, .. }) => {
                // Login successfull
                self.logged_in = true;
                Ok(())
            }
            ReceiveMsg::Response(Response::Login { result: false, .. }) => Err(Error::LoginFailed),
            _ => Err(Error::MessageHandlerNotImplemented),
        }
    }

    async fn send(&mut self, msg: SendMsg) {
        println!("[GrandMa2] Sending: {:?}", msg);
        let msg_string = serde_json::to_string(&msg).unwrap();
        let _ = self
            .ws_sink
            .as_mut()
            .expect("WS has no sink!")
            .send(Message::Text(msg_string)).await.unwrap();
    }

    async fn send_session(&mut self) {
        let session_msg = SendMsg::Session {
            session: self.session_id,
        };
        self.send(session_msg).await;
    }

    async fn send_login(&mut self) {
        let login_msg = SendMsg::Request(Request::Login {
            username: self.username.clone(),
            password: self.password.clone(),
            max_requests: 10,
            session: self.session_id,
        });
        self.send(login_msg).await;
    }

    pub async fn close_connection(&mut self) {
        let close_msg = SendMsg::Request(Request::Close {
            session: self.session_id,
            max_requests: 10,
        });
        self.send(close_msg).await;
    }
}

impl Drop for GrandMa2Client {
    fn drop(&mut self) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        // Block on the async function
        rt.block_on(async {
            self.close_connection().await;
        });
    }
}
