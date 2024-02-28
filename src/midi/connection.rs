use std::sync::{mpsc, Arc};

use midir::MidiOutputConnection;

#[derive(Debug, PartialEq, Eq)]
pub enum DeviceType {
    XTouch,
    Launchpad,
}

pub struct MidiConnection {
    // name: String,
    device_type: DeviceType,
    sender: mpsc::Sender<Arc<[u8]>>,
}

impl MidiConnection {
    pub fn new(connection: MidiOutputConnection, device_type: DeviceType) -> Self {
        let (sender, receiver) = mpsc::channel();
        let _handle = tokio::spawn(async move {
            Self::run_connection(connection, receiver).await;
        });

        MidiConnection {
            device_type,
            sender,
        }
    }

    pub fn send(&self, data: Arc<[u8]>) -> Result<(), mpsc::SendError<Arc<[u8]>>> {
        self.sender.send(data)
    }

    pub fn is_type(&self, other_type: &DeviceType) -> bool {
        self.device_type == *other_type
    }

    async fn run_connection(
        mut connection: MidiOutputConnection,
        receiver: mpsc::Receiver<Arc<[u8]>>,
    ) {
        loop {
            let msg = receiver.recv().unwrap();
            connection.send(&msg).unwrap();
        }
    }
}
