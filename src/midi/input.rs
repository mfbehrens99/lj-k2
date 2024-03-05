use midi_parse::MidiMessage;
use midir::{Ignore, MidiInputConnection};
use std::{collections::HashSet, time::Duration};
use tokio::{sync::mpsc, time::sleep};

use crate::midi::connection::MessageMidi;

use super::DeviceType;

pub struct MidiIn {
    connected_devices: HashSet<String>,
    connections: Vec<MidiInputConnection<mpsc::Sender<MessageMidi>>>,
    tx_incoming_msgs: mpsc::Sender<MessageMidi>,
}

impl MidiIn {
    pub fn new(tx_incoming_msgs: mpsc::Sender<MessageMidi>) -> MidiIn {
        MidiIn {
            connected_devices: HashSet::new(),
            connections: Vec::new(),
            tx_incoming_msgs,
        }
    }

    pub async fn run(&mut self) {
        loop {
            let mut midi_in = midir::MidiInput::new("LJK2 midi in").unwrap();
            midi_in.ignore(Ignore::None);

            let ports = midi_in.ports();
            for port in ports.iter() {
                let port_name = midi_in.port_name(port).unwrap();
                let device_type = port_name.split(':').next().unwrap();

                if !self.connected_devices.contains(&port_name) {
                    println!("New input device detected: {}", port_name);
                    self.connected_devices.insert(port_name.clone());

                    let device_type = match device_type {
                        "Launchpad MK2" => DeviceType::Launchpad,
                        "X-TOUCH COMPACT" => DeviceType::XTouch,
                        _ => {
                            println!("Unknown midi device, not connecting to it.");
                            break;
                        }
                    };

                    let _conn_in = midi_in
                        .connect(
                            port,
                            "LJK2 midi in device",
                            move |_timestamp, message, data| {
                                handle_midi_message(message, data, device_type)
                            },
                            self.tx_incoming_msgs.clone(),
                        )
                        .unwrap();
                    self.connections.push(_conn_in);
                    break;
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
    }
}

fn handle_midi_message(
    message: &[u8],
    data: &mut mpsc::Sender<MessageMidi>,
    device_type: DeviceType,
) {
    let msg = MidiMessage::from(message).unwrap();
    println!("Launchpad: {:?}", msg);
    data.try_send(MessageMidi {
        midi_msg: msg,
        device_type,
    })
    .unwrap();
}
