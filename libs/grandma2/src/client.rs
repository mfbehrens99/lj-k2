extern crate md5;

use std::{io::Write, net::TcpStream};

use tokio::sync::mpsc;
use tungstenite::{stream::MaybeTlsStream, WebSocket};
use url::Url;

use crate::messages::{ReceiveMsg, Request, Response, SendMsg};

const MAX_REQUESTS: u8 = 9;

#[derive(Debug)]
enum Error {
    MessageHandlerNotImplemented,
    GrandMA2Connection,
    WebRemoteDisabled,
    LoginFailed,
}

enum MaRequest {}

#[derive(Debug)]
pub struct GrandMa2 {
    url: Url,
    username: String,
    password: String,
    websocket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    logged_in: bool,
    session_id: i8,
    num_requests: u8,
}

impl GrandMa2 {
    pub fn new<T, U>(ip: T, username: T, password: U) -> GrandMa2
    where
        T: Into<String>,
        U: AsRef<[u8]>,
    {
        GrandMa2 {
            url: Url::parse(&ip.into()).unwrap(),
            username: username.into(),
            password: format!("{:x}", md5::compute(password)),
            websocket: None,
            logged_in: false,
            session_id: 0,
            num_requests: 0,
        }
    }

    pub fn connect(&mut self) {
        let (ws_stream, _response) =
            tungstenite::connect(self.url.clone()).expect("Failed to connect");
        self.websocket = Some(ws_stream);
    }

    pub async fn run(&mut self) {
        let msg_raw = self
            .websocket
            .as_mut()
            .expect("GrandMA is not connected to any websocket!")
            .read()
            .unwrap();
        let msg_string = msg_raw.into_text().unwrap();
        let msg: ReceiveMsg = serde_json::from_str(&msg_string)
            .expect(&format!("Could not parse message: '{}'", msg_string));
        println!("[GrandMa2] Receive: {:?}", msg);
        self.handle_message(msg).unwrap();
    }

    fn handle_message(&mut self, msg: ReceiveMsg) -> Result<(), Error> {
        // Handle weird GrandMa behaviour
        self.num_requests += 1;
        if self.num_requests >= MAX_REQUESTS {
            self.send_session()
        }

        match msg {
            ReceiveMsg::Session { session, .. } => match session {
                -1 => Err(Error::WebRemoteDisabled),
                0 => Err(Error::GrandMA2Connection),
                _ => {
                    self.session_id = session;
                    Ok(())
                }
            },
            ReceiveMsg::Status { status, app_type }
                if status.as_ref() == "server ready" && app_type.as_ref() == "gma2" =>
            {
                self.send_session();
                Ok(())
            }
            ReceiveMsg::ForceLogin { force_login: true } => {
                self.send_login();
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

    fn send(&mut self, msg: SendMsg) {
        if let Some(websocket) = self.websocket.as_mut() {
            println!("[GrandMa2] Sending: {:?}", msg);
            let msg_string = serde_json::to_string(&msg).unwrap();
            websocket.get_mut().write(msg_string.as_bytes()).unwrap();
        }
    }

    fn send_session(&mut self) {
        let session_msg = SendMsg::Session {
            session: self.session_id,
        };
        self.send(session_msg)
    }

    fn send_login(&mut self) {
        let login_msg = SendMsg::Request(Request::Login {
            username: self.username.clone(),
            password: self.password.clone(),
            max_requests: 10,
            session: self.session_id,
        });
        self.send(login_msg);
    }

    pub fn close_connection(&mut self) {
        let close_msg = SendMsg::Request(Request::Close {
            session: self.session_id,
            max_requests: 10,
        });
        self.send(close_msg)
    }
}

impl Drop for GrandMa2 {
    fn drop(&mut self) {
        self.close_connection();
    }
}
