mod data;
pub mod messages;
mod presets;

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

pub struct Frontend {
    listener: TcpListener,
    senders: Vec<Sender<String>>,
}

impl Frontend {
    pub async fn new(address: &str) -> Frontend {
        let listener = TcpListener::bind(&address)
            .await
            .unwrap_or_else(|_| panic!("Can't listen on {:}", address));
        println!("Listening on: {}", address);
        Frontend {
            listener,
            senders: Vec::new(),
        }
    }

    pub async fn listen(&mut self) {
        while let Ok((stream, _)) = self.listener.accept().await {
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
                        self.handle_message(msg);
                    }
                }
                // Heartbeat
                _ = interval.tick() => {
                    // self.websocket.send(Message::Text(json!({"type": "heartbeat"}).to_string())).await?;
                }
            }
            // Receive message from channel
            if let Ok(msg) = self.channel_rx.try_recv() {
                self.websocket.send(Message::Text(msg)).await?;
            }
        }
    }

    fn handle_message(&self, msg: Result<Message, Error>) {
        let msg = msg.unwrap();
        if msg.is_text() || msg.is_binary() {
            match serde_json::from_str::<ReceiveMessage>(msg.clone().into_text().unwrap().as_str())
            {
                Ok(msg) => {
                    println!("Recieved message: {:?}", msg);
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
}

#[cfg(test)]
mod tests {
    // Your test cases here
}
