use std::time::Duration;

use tokio::{net::TcpStream, sync::mpsc};

use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Error;

use crate::frontend::FrontendClient;

use super::messages::{ReceiveMessage, SendMessage};

pub struct Server {
    tx_outgoing_msgs: mpsc::Sender<SendMessage>,
    rx_outgoing_msgs: mpsc::Receiver<SendMessage>,
    tx_incoming_msgs: mpsc::Sender<ReceiveMessage>,
    address: String,
    client_senders: Vec<mpsc::Sender<String>>,
}

impl Server {
    pub fn new<T>(address: T, tx_incoming_msgs: mpsc::Sender<ReceiveMessage>) -> Self
    where
        T: Into<String>,
    {
        let (tx_outgoing_msgs, rx_outgoing_msgs) = mpsc::channel(100);
        Server {
            tx_outgoing_msgs,
            rx_outgoing_msgs,
            tx_incoming_msgs,
            address: address.into(),
            client_senders: Vec::new(),
        }
    }

    pub async fn run(&mut self) {
        let listener = TcpListener::bind(&self.address).await.unwrap();
        println!("Listening for frontend on {}", self.address);

        let mut heartbeat_interval = tokio::time::interval(Duration::from_secs(5));

        loop {
            tokio::select! {
                Ok((stream, _)) = listener.accept() => {
                    self.start_connection(stream).await;
                }
                Some(msg) = self.rx_outgoing_msgs.recv() => {
                    self.send(msg).await;
                }
                _ = heartbeat_interval.tick() => {
                    self.send(SendMessage::Heartbeat).await;
                }
            };
        }
    }

    async fn start_connection(&mut self, stream: TcpStream) {
        let (tx_outgoing_msgs, rx_outgoing_msgs) = mpsc::channel(100);
        let mut client =
            FrontendClient::new(stream, rx_outgoing_msgs, self.tx_incoming_msgs.clone()).await;
        self.client_senders.push(tx_outgoing_msgs);

        // Run new connection
        tokio::task::spawn(async move {
            if let Err(error) = client.run().await {
                match error {
                    Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                    err => println!("Error processing connection: {}", err),
                }
            }
        });
    }

    async fn send(&self, msg: SendMessage) {
        let msg_str: String = serde_json::to_string(&msg).unwrap();
        println!("Sending out: {}", msg_str);
        for sender in self.client_senders.iter() {
            sender.send(msg_str.clone()).await.unwrap();
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<SendMessage> {
        self.tx_outgoing_msgs.clone()
    }
}
