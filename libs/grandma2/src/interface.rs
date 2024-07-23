
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::client::{GrandMa2Client, MaEvent, MaRequest};

#[derive(Debug)]
pub struct GrandMa2 {
    sender: UnboundedSender<MaRequest>,
    receiver: UnboundedReceiver<MaEvent>,
}

impl GrandMa2 {
    pub fn new<T, U>(url: T, username: T, password: U) -> Self
    where
        T: Into<String>,
        U: AsRef<[u8]>,
    {

        // Create channels into the run method
        let (tx_request, rx_request) = unbounded_channel::<MaRequest>();
        let (tx_event, rx_event) = unbounded_channel::<MaEvent>();

        let mut client = GrandMa2Client::new(url, username, password, rx_request, tx_event);

        tokio::spawn(async move {
            client.run().await;
        });

        Self {
            sender: tx_request,
            receiver: rx_event,
        }
    }

    pub async fn recv(&mut self) -> Option<MaEvent> {
        self.receiver.recv().await
    }
}

#[cfg(test)]
mod test {
    todo!();
}
