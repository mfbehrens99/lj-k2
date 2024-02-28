use std::sync::Arc;

use midi_parse::MidiMessage;

use super::connection::DeviceType;
use super::connection::MidiConnection;

pub struct MidiInterface {
    devices: Vec<MidiConnection>,
}

impl MidiInterface {
    pub fn new() -> Self {
        MidiInterface {
            devices: Vec::new(),
        }
    }

    pub fn send(&self, msg: MidiMessage, device_type: DeviceType) {
        let data: Arc<[u8]> = msg.to_bytes().into();

        for device in self.devices.iter() {
            // Maybe send it in parallel?
            if device.is_type(&device_type) {
                device.send(data.clone()).unwrap();
            }
        }
    }

    pub fn add(&mut self, connection: MidiConnection) {
        self.devices.push(connection);
    }
}
