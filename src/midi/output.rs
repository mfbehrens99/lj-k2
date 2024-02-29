use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::sleep;

use crate::LJK2;

use super::connection::{DeviceType, MidiConnection};
use super::MidiInterface;

pub struct MidiOut {
    midi_interface: Arc<Mutex<MidiInterface>>,
    connected_devices: HashSet<String>,
}

impl MidiOut {
    pub fn new(main: &LJK2) -> MidiOut {
        MidiOut {
            midi_interface: main.midi.clone(),
            connected_devices: HashSet::new(),
        }
    }

    pub async fn listen(&mut self) {
        loop {
            let midi_out = midir::MidiOutput::new("LJK2 midi out").unwrap();
            // midi_out.ignore(Ignore::None);

            let ports = midi_out.ports();
            for port in ports.iter() {
                let port_name = midi_out.port_name(port).unwrap();
                let device_type = port_name.split(":").next().unwrap();

                if !self.connected_devices.contains(&port_name) {
                    println!("New output device detected: {}", port_name);
                    self.connected_devices.insert(port_name.clone());

                    let device_type: DeviceType = match device_type {
                        "Launchpad MK2" => DeviceType::Launchpad,
                        "X-TOUCH COMPACT" => DeviceType::XTouch,
                        _ => {
                            println!("Unknown midi device, not connecting to it.");
                            break;
                        }
                    };

                    let connection = midi_out.connect(port, "LJK2 midi out device").unwrap();
                    let midi_conn = MidiConnection::new(connection, device_type);

                    let mut devices = self.midi_interface.lock().unwrap();
                    devices.add(midi_conn);

                    break;
                }
            }
            sleep(Duration::from_millis(500)).await;
        }
    }
}
