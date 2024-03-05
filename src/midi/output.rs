use midi_parse::MidiMessage;
use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio::sync::mpsc;

use crate::midi::connection::MessageMidi;

use super::connection::{DeviceType, MidiConnection};

pub struct MidiOut {
    rx_outgoing_msgs: mpsc::Receiver<MessageMidi>,
    tx_outgoing_msgs: mpsc::Sender<MessageMidi>,
    devices_set: HashSet<String>,
    connections: Vec<MidiConnection>,
}

impl MidiOut {
    pub fn new() -> MidiOut {
        let (tx_outgoing_msgs, rx_outgoing_msgs) = mpsc::channel(100);
        MidiOut {
            rx_outgoing_msgs,
            tx_outgoing_msgs,
            devices_set: HashSet::new(),
            connections: Vec::new(),
        }
    }

    pub async fn run(&mut self) {
        let mut interval = tokio::time::interval(Duration::from_secs(5));

        loop {
            tokio::select! {
                // Receive message outgoing messages
                msg = self.rx_outgoing_msgs.recv() => {
                    if let Some(msg) = msg {
                        self.send(msg.midi_msg, msg.device_type);
                    }
                }
                // Detect new devices
                _ = interval.tick() => {
                    let (port, conn) = tokio::task::spawn(
                        MidiOut::scan_devices(self.devices_set.clone())
                    ).await.unwrap();

                    if let Some(port) = port {
                        self.devices_set.insert(port);
                        if let Some(conn) = conn {
                            self.connections.push(conn)
                        }
                    }
                }
            }
        }
    }

    pub fn send(&self, msg: MidiMessage, device_type: DeviceType) {
        let data: Arc<[u8]> = msg.to_bytes().into();

        for conn in self.connections.iter() {
            // Maybe send it in parallel?
            if conn.is_type(&device_type) {
                conn.send(data.clone()).unwrap();
            }
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<MessageMidi> {
        self.tx_outgoing_msgs.clone()
    }

    async fn scan_devices(
        devices_set: HashSet<String>,
    ) -> (Option<String>, Option<MidiConnection>) {
        let midi_out = midir::MidiOutput::new("LJK2 midi out").unwrap();

        let ports = midi_out.ports();
        for port in ports.iter() {
            let port_name = midi_out.port_name(port).unwrap();
            let device_type = port_name.split(':').next().unwrap();

            if !devices_set.contains(&port_name) {
                println!("New output device detected: {}", port_name);
                // devices_set.insert(port_name.clone());

                let device_type: DeviceType = match device_type {
                    "Launchpad MK2" => DeviceType::Launchpad,
                    "X-TOUCH COMPACT" => DeviceType::XTouch,
                    _ => {
                        println!("Unknown midi device, not connecting to it.");
                        return (Some(port_name), None);
                    }
                };

                let connection = midi_out.connect(port, "LJK2 midi out device").unwrap();
                let midi_conn = MidiConnection::new(&port_name, connection, device_type);

                // self.connections.push(midi_conn);
                return (Some(port_name), Some(midi_conn));
            }
        }
        (None, None)
    }
}
