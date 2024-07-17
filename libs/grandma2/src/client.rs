extern crate md5;

use std::{
    io::Write,
    net::{IpAddr, TcpStream},
};

use tungstenite::{stream::MaybeTlsStream, WebSocket};
use url::Url;

use crate::messages::{SendMsg, ReceiveMsg, Request};

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
        println!("{:?}", self);
        let (ws_stream, _response) =
            tungstenite::connect(self.url.clone()).expect("Failed to connect");
        self.websocket = Some(ws_stream);

        self.login();
    }

    pub async fn run(&mut self) {
        let msg = self
            .websocket
            .as_mut()
            .expect("GrandMA is not connected to any websocket!")
            .read()
            .unwrap();
        let msg_parsed: ReceiveMsg =
            serde_json::from_str(msg.into_text().unwrap().as_str()).unwrap();
        println!("{:?}", msg_parsed)
    }

    fn login(&mut self) {
        if let Some(websocket) = self.websocket.as_mut() {
            let login_msg = SendMsg::Request(Request::Login {
                username: self.username.clone(),
                password: self.password.clone(),
                max_requests: 10,
                session: self.session_id,
            });
            let msg_string = serde_json::to_string(&login_msg).unwrap();
            websocket.get_mut().write(msg_string.as_bytes()).unwrap();
        }
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
