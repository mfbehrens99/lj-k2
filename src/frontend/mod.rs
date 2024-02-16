mod data;
pub mod messages;
mod presets;

pub use messages::*;
use serde_json::json;

use futures_util::{SinkExt, StreamExt};
use std::{net::SocketAddr, time::Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};

pub struct Frontend {
    listener: TcpListener,
    // senders: Arc<Mutex<Vec<SplitSink<WebSocketStream<TcpStream>, Message>>>>,
}

impl Frontend {
    pub async fn new(address: &str) -> Frontend {
        let listener = TcpListener::bind(&address)
            .await
            .unwrap_or_else(|_| panic!("Can't listen on {:}", address));
        println!("Listening on: {}", address);
        Frontend {
            listener,
            // senders: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn listen(&self) {
        while let Ok((stream, _)) = self.listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("connected streams should have a peer address");
            println!("Peer address: {}", peer);

            self.accept_connection(peer, stream).await;
        }
    }

    async fn accept_connection(&self, peer: SocketAddr, stream: TcpStream) {
        if let Err(e) = self.handle_connection(peer, stream).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => println!("Error processing connection: {}", err),
            }
        }
    }

    async fn handle_connection(&self, peer: SocketAddr, stream: TcpStream) -> Result<()> {
        let ws_stream = accept_async(stream).await.expect("Failed to accept");
        println!("New WebSocket connection: {}", peer);
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let mut interval = tokio::time::interval(Duration::from_secs(5));

        // Echo incoming WebSocket messages and send a message periodically every second.

        loop {
            tokio::select! {
                msg = ws_receiver.next() => {
                    match msg {
                        Some(msg) => {
                            let msg = msg?;
                            if msg.is_text() || msg.is_binary() {
                                // echo
                                ws_sender.send(msg.clone()).await?;
                                match serde_json::from_str::<ReceiveMessage>(msg.clone().into_text()?.as_str()) {
                                    Ok(msg) => {println!("Recieved message: {:?}", msg);},
                                    Err(err) => {println!("Could not read message '{:}': {:}", msg.into_text().unwrap(), err);},
                                }
                            } else if msg.is_close() {
                                println!("Closed connection to {:}", peer);
                                break;
                            }
                        }
                        None => break,
                    }
                }
                _ = interval.tick() => {
                    ws_sender.send(Message::Text(json!({"type": "heartbeat"}).to_string())).await?;
                }
            }
        }

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    // Your test cases here
}
