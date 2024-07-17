extern crate md5;

use std::{io::Write, net::TcpStream};

use tungstenite::{stream::MaybeTlsStream, WebSocket};
use url::Url;

use crate::messages::{ReceiveMsg, Request, SendMsg};

#[derive(Debug)]
pub struct GrandMa2 {
    url: Url,
    username: String,
    password: String,
    websocket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    session_id: u8,
    // ws_receiver: WsReceiver,
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
            session_id: 5,
        }
    }

    pub fn connect(&mut self) {
        let (ws_stream, _response) =
            tungstenite::connect(self.url.clone()).expect("Failed to connect");
        self.websocket = Some(ws_stream);

        self.login();
    }

    pub async fn run(&mut self) {
        let msg_raw = self
            .websocket
            .as_mut()
            .expect("GrandMA is not connected to any websocket!")
            .read()
            .unwrap();
        let msg_string = msg_raw.into_text().unwrap();
        let msg: ReceiveMsg = serde_json::from_str(&msg_string).expect(&format!("Could not parse message: '{}'", msg_string));
        println!("{:?}", msg);
        self.handle_message(msg)
    }

    fn handle_message(&mut self, msg: ReceiveMsg) {
        match msg {
            ReceiveMsg::Status {status, app_type} => {
                if status == "server ready" && app_type == "gma2" {
                    self.login();
                }
            }
            _ => {}
        }
    }

    fn send(&mut self, msg: SendMsg) {
        if let Some(websocket) = self.websocket.as_mut() {
            let msg_string = serde_json::to_string(&msg).unwrap();
            print!("[GrandMa2] Sending '{}'", msg_string);
            websocket.get_mut().write(msg_string.as_bytes()).unwrap();
        }
    }

    fn login(&mut self) {
        let login_msg = SendMsg::Request(Request::Login {
            username: self.username.clone(),
            password: self.password.clone(),
            max_requests: 10,
            session: self.session_id,
        });
        self.send(login_msg);
    }

    pub fn close_connection(&mut self) {
        if let Some(websocket) = self.websocket.as_mut() {
            let close_msg = SendMsg::Request(Request::Close {
                session: self.session_id,
                max_requests: 10,
            });
            let msg_string = serde_json::to_string(&close_msg).unwrap();
            websocket.get_mut().write(msg_string.as_bytes()).unwrap();
        }
    }
}

impl Drop for GrandMa2 {
    fn drop(&mut self) {
        self.close_connection();
    }
}
