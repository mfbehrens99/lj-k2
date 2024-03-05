use tokio_tungstenite::WebSocketStream;

use tokio::sync::mpsc;

use futures_util::{SinkExt, StreamExt};
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};

use crate::frontend::{
    data::{self, Icon},
    messages, ReceiveMessage,
};

pub struct FrontendClient {
    websocket: WebSocketStream<TcpStream>,
    rx_outgoing_msgs: mpsc::Receiver<String>,
    tx_incoming_msgs: mpsc::Sender<ReceiveMessage>,
    peer: SocketAddr,
}

impl FrontendClient {
    pub async fn new(
        stream: TcpStream,
        rx_outgoing_msgs: mpsc::Receiver<String>,
        tx_incoming_msgs: mpsc::Sender<ReceiveMessage>,
    ) -> Self {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        let websocket = accept_async(stream).await.expect("Failed to accept");
        println!("Peer address: {}", peer);
        FrontendClient {
            websocket,
            rx_outgoing_msgs,
            tx_incoming_msgs,
            peer,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("New Websocket connection: {}", self.peer);
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        loop {
            tokio::select! {
                // Receive message from websocket
                msg = self.websocket.next() => {
                    if let Some(msg) = msg {
                        self.handle_message(msg).await;
                    }
                }
                // Receive message from Interface
                msg = self.rx_outgoing_msgs.recv() => {
                    if let Some(msg) = msg {
                        self.websocket.send(Message::Text(msg)).await.unwrap();
                    }
                }
                // Heartbeat
                _ = interval.tick() => {
                    self.websocket.send(Message::Text(serde_json::json!({"type": "heartbeat"}).to_string())).await?;
                }
            }
        }
    }

    async fn handle_message(&mut self, msg: Result<Message, Error>) {
        let msg = msg.unwrap();
        if msg.is_text() || msg.is_binary() {
            match serde_json::from_str::<ReceiveMessage>(msg.clone().into_text().unwrap().as_str())
            {
                Ok(msg) => {
                    println!("Received message: {:?}", msg);
                    match msg {
                        ReceiveMessage::RequestPresetCategoryDefinitions => {
                            self.send(messages::SendMessage::SendPresetCategoryDefinitions {
                                items: Box::new([
                                    data::PresetCategory::new(0, "Bar"),
                                    data::PresetCategory::new(1, "Tresen"),
                                ]),
                            })
                            .await;
                        }
                        ReceiveMessage::RequestPresetButtonDefinitions => {
                            use data::PresetButton as PB;
                            self.send(messages::SendMessage::SendPresetButtonDefinitions {
                                items: Box::new([
                                    PB::new("Bar Chill 1", 0, 0, Icon::Chill, "#c06541"),
                                    PB::new("Bar Chill 2", 0, 1, Icon::Chill, "#c06541"),
                                    PB::new("Bar Party 1", 0, 2, Icon::Party, "#41c0a6"),
                                    PB::new("Bar Party 2", 0, 3, Icon::Party, "#41c0a6"),
                                    PB::new("Bar Rave 1", 0, 4, Icon::Rave, "#a541d4"),
                                    PB::new("Bar Rave 2", 0, 5, Icon::Rave, "#a541d4"),
                                    PB::new("Bar Putzlich", 0, 6, Icon::Sun, "#e2d195"),
                                    PB::new("Bar Aus", 0, 7, Icon::Off, "#38365a"),
                                    PB::new("Tresen Chill", 1, 0, Icon::Chill, "#c06541"),
                                    PB::new("Tresen Party", 1, 1, Icon::Party, "#41c0a6"),
                                    PB::new("Tresen Rave", 1, 2, Icon::Rave, "#a541d4"),
                                    PB::new("Tresen Rainbow", 1, 3, Icon::Rainbow, "#a541d4"),
                                    PB::new("Tresen Putzlicht", 1, 4, Icon::Sun, "#e2d195"),
                                    PB::new("Tresen Aus", 1, 5, Icon::Off, "#38365a"),
                                ]),
                            })
                            .await;
                        }
                        ReceiveMessage::RequestFaderDefinitions => {
                            use data::Fader as F;
                            self.send(messages::SendMessage::SendFaderDefinitions {
                                items: Box::new([
                                    F::new("LED Bars", 0, 0, Icon::LEDBars, "#95724f"),
                                    F::new("Sunstripes", 0, 1, Icon::Sunstrip, "#508746"),
                                    F::new("Moving Heads", 0, 2, Icon::MovingHead, "#968d3f"),
                                    F::new("Tresen", 0, 3, Icon::CounterFront, "#95724f"),
                                    F::new("Hexagons", 0, 4, Icon::Hexagon, "#945a5f"),
                                    F::new("Strobes", 0, 5, Icon::Sun, "#94497a"),
                                    F::new("H-Bars", 0, 6, Icon::LEDBars, "#8d418e"),
                                    F::new("Pointies", 0, 7, Icon::MovingHead, "#72429a"),
                                ]),
                            })
                            .await;
                        }
                        _ => (),
                    }
                    self.tx_incoming_msgs.send(msg).await.unwrap();
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

    pub async fn send(&mut self, msg: messages::SendMessage) {
        let string = serde_json::to_string(&msg).unwrap();
        println!("Sending '{:}'", string);
        let _ = self.websocket.send(Message::Text(string)).await;
    }

    // pub async fn close(&mut self) {
    //     todo!()
    // }
}
