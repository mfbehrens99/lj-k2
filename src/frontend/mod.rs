mod data;
pub mod messages;
mod presets;
#[cfg(test)]
mod tests;

pub use messages::*;
use tokio_tungstenite::WebSocketStream;

use std::sync::mpsc::{channel, Receiver, Sender};

use futures_util::{SinkExt, StreamExt};
use std::{net::SocketAddr, time::Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};

use crate::frontend::data::Icon;
use crate::LJK2;

pub struct Frontend {
    main: LJK2,
    address: String,
    senders: Vec<Sender<String>>,
}

impl Frontend {
    pub fn new(main: &LJK2, address: &str) -> Frontend {
        Frontend {
            main: main.clone(),
            address: address.to_owned(),
            senders: Vec::new(),
        }
    }

    pub async fn listen(&mut self) {
        let listener = TcpListener::bind(&self.address)
            .await
            .unwrap_or_else(|_| panic!("Can't listen on {:}", self.address));
        println!("Listening for frontend on {}", self.address);
        while let Ok((stream, _)) = listener.accept().await {
            self.accept_connection(stream).await;
        }
    }

    async fn accept_connection(&mut self, stream: TcpStream) {
        let (channel_tx, channel_rx) = channel::<String>();
        self.senders.push(channel_tx);
        let mut f = FrontendClient::new(stream, channel_rx).await;
        if let Err(e) = f.handle_connection().await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => println!("Error processing connection: {}", err),
            }
        }
    }
}

pub struct FrontendClient {
    websocket: WebSocketStream<TcpStream>,
    channel_rx: Receiver<String>,
    peer: SocketAddr,
}

impl FrontendClient {
    async fn new(stream: TcpStream, channel_rx: Receiver<String>) -> Self {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        let websocket = accept_async(stream).await.expect("Failed to accept");
        println!("Peer address: {}", peer);
        FrontendClient {
            websocket,
            channel_rx,
            peer,
        }
    }

    async fn handle_connection(&mut self) -> Result<()> {
        println!("New WebSocket connection: {}", self.peer);
        // let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        // self.senders.insert(peer, ws_stream);
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        loop {
            tokio::select! {
                // Receive message from websocket
                msg = self.websocket.next() => {
                    if let Some(msg) = msg {
                        self.handle_message(msg).await;
                    }
                }
                // Heartbeat
                _ = interval.tick() => {
                    self.websocket.send(Message::Text(serde_json::json!({"type": "heartbeat"}).to_string())).await?;
                }
            }
            // Receive message from channel
            if let Ok(msg) = self.channel_rx.try_recv() {
                self.websocket.send(Message::Text(msg)).await?;
            }
        }
    }

    async fn handle_message(&mut self, msg: Result<Message, Error>) {
        // println!("Handeling mesage {:?}", msg);
        let msg = msg.unwrap();
        if msg.is_text() || msg.is_binary() {
            match serde_json::from_str::<ReceiveMessage>(msg.clone().into_text().unwrap().as_str())
            {
                Ok(msg) => {
                    println!("Recieved message: {:?}", msg);
                    match msg {
                        ReceiveMessage::RequestPresetCategoryDefinitions => {
                            self.send(messages::SendMessage::SendPresetCategoryDefinitions {
                                items: &[
                                    data::PresetCategory::new(0, "Bar"),
                                    data::PresetCategory::new(1, "Tresen"),
                                ],
                            })
                            .await;
                        }
                        ReceiveMessage::RequestPresetButtonDefinitions => {
                            self.send(messages::SendMessage::SendPresetButtonDefinitions {
                                items: &[
                                    data::PresetButton::new(
                                        "Bar Chill 1",
                                        0,
                                        0,
                                        Icon::Chill,
                                        "#c06541",
                                    ),
                                    data::PresetButton::new(
                                        "Bar Chill 2",
                                        0,
                                        1,
                                        Icon::Chill,
                                        "#c06541",
                                    ),
                                    data::PresetButton::new(
                                        "Bar Party 1",
                                        0,
                                        2,
                                        Icon::Party,
                                        "#41c0a6",
                                    ),
                                    data::PresetButton::new(
                                        "Bar Party 2",
                                        0,
                                        3,
                                        Icon::Party,
                                        "#41c0a6",
                                    ),
                                    data::PresetButton::new(
                                        "Bar Rave 1",
                                        0,
                                        4,
                                        Icon::Rave,
                                        "#a541d4",
                                    ),
                                    data::PresetButton::new(
                                        "Bar Rave 2",
                                        0,
                                        5,
                                        Icon::Rave,
                                        "#a541d4",
                                    ),
                                    data::PresetButton::new(
                                        "Bar Putzlich",
                                        0,
                                        6,
                                        Icon::Sun,
                                        "#e2d195",
                                    ),
                                    data::PresetButton::new("Bar Aus", 0, 7, Icon::Off, "#38365a"),
                                    data::PresetButton::new(
                                        "Tresen Chill",
                                        1,
                                        0,
                                        Icon::Chill,
                                        "#c06541",
                                    ),
                                    data::PresetButton::new(
                                        "Tresen Party",
                                        1,
                                        1,
                                        Icon::Party,
                                        "#41c0a6",
                                    ),
                                    data::PresetButton::new(
                                        "Tresen Rave",
                                        1,
                                        2,
                                        Icon::Rave,
                                        "#a541d4",
                                    ),
                                    data::PresetButton::new(
                                        "Tresen Rainbow",
                                        1,
                                        3,
                                        Icon::Rainbow,
                                        "#a541d4",
                                    ),
                                    data::PresetButton::new(
                                        "Tresen Putzlicht",
                                        1,
                                        4,
                                        Icon::Sun,
                                        "#e2d195",
                                    ),
                                    data::PresetButton::new(
                                        "Tresen Aus",
                                        1,
                                        5,
                                        Icon::Off,
                                        "#38365a",
                                    ),
                                ],
                            })
                            .await;
                        }
                        ReceiveMessage::RequestFaderDefinitions => {
                            self.send(SendMessage::SendFaderDefinitions {
                                items: &[
                                    data::Fader::new("LED Bars", 0, 0, Icon::LEDBars, "#95724f"),
                                    data::Fader::new("Sunstripes", 0, 1, Icon::Sunstrip, "#508746"),
                                    data::Fader::new(
                                        "Moving Heads",
                                        0,
                                        2,
                                        Icon::MovingHead,
                                        "#968d3f",
                                    ),
                                    data::Fader::new("Tresen", 0, 3, Icon::CounterFront, "#95724f"),
                                    data::Fader::new("Hexagons", 0, 4, Icon::Hexagon, "#945a5f"),
                                    data::Fader::new("Strobes", 0, 5, Icon::Sun, "#94497a"),
                                    data::Fader::new("H-Bars", 0, 6, Icon::LEDBars, "#8d418e"),
                                    data::Fader::new("Pointies", 0, 7, Icon::MovingHead, "#72429a"),
                                ],
                            })
                            .await;
                        }
                        _ => (),
                    }
                }
                Err(err) => {
                    println!(
                        "Could not read message '{:}': {:}",
                        msg.into_text().unwrap(),
                        err
                    );
                }
            }
        } else if msg.is_close() {
            println!("Closed connection to {:}", self.peer);
        }
    }

    async fn send<'a>(&mut self, msg: SendMessage<'a>) {
        let string = serde_json::to_string(&msg).unwrap();
        println!("Sending '{:}'", string);
        let _ = self.websocket.send(Message::Text(string)).await;
    }
}
