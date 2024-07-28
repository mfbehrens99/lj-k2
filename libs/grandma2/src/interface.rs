extern crate md5;

use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::{client::GrandMa2Client, Ma2Error, Result};

use super::interface_msg::{MaEvent, MaRequest};

/// The main GrandMa2 struct to connect and control GrandMa2
///
/// ```
/// let grandma = GrandMa2::new("ws://127.0.0.1", "remote", "password");
/// grandma.connect().await.unwrap();
/// ```
#[derive(Debug)]
pub struct GrandMa2 {
    url: String,
    username: String,
    password: String,
    tx_request: Option<UnboundedSender<MaRequest>>,
    rx_event: Option<UnboundedReceiver<MaEvent>>,
    join_handler: Option<JoinHandle<()>>,
}

impl GrandMa2 {
    pub fn new(
        url: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            url: url.into(),
            username: username.into(),
            password: password.into(),
            tx_request: None,
            rx_event: None,
            join_handler: None,
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        // Connect to the websocket
        let (ws_stream, _response) =
            tokio_tungstenite::connect_async(&self.url)
                .await
                .map_err(|tungstenite_error| Ma2Error::FailedToConnect {
                    url: self.url.clone(),
                    tungstenite_error,
                })?;

        // Create channels to communicate between the client thread and the interface
        let (tx_request, rx_request) = unbounded_channel::<MaRequest>();
        let (tx_event, rx_event) = unbounded_channel::<MaEvent>();

        self.tx_request = Some(tx_request);
        self.rx_event = Some(rx_event);

        let mut client = GrandMa2Client::new(
            ws_stream,
            rx_request,
            tx_event,
            self.username.clone(),
            self.password.clone(),
        );

        let join_handle = tokio::spawn(async move {
            client.run().await;
        });

        self.join_handler = Some(join_handle);
        Ok(())
    }

    pub async fn recv(&mut self) -> Result<MaEvent> {
        if let Some(rx_event) = self.rx_event.as_mut() {
            return rx_event.recv().await.ok_or(Ma2Error::EventChannelClosed);
        }
        Err(Ma2Error::NotYetConnected)
    }

    pub fn close_connection(&mut self) {
        // Only close connection if connection has been opened
        if let Some(tx_request) = self.tx_request.as_mut() {
            let close_msg = MaRequest::Disconnect;
            tx_request.send(close_msg).unwrap();
        }
    }
}

impl Drop for GrandMa2 {
    fn drop(&mut self) {
        self.close_connection();
        if let Some(join_handler) = self.join_handler.take() {
            while !join_handler.is_finished() {}
        }
    }
}
