use midi_parse::{Channel, MidiMessage, Note, Velocity};
use midir::{Ignore, MidiInputConnection};
use std::{collections::HashSet, time::Duration};
use tokio::time::sleep;

use crate::LJK2;

pub struct MidiIn {
    main: LJK2,
    connected_devices: HashSet<String>,
    connections: Vec<MidiInputConnection<LJK2>>,
}

impl MidiIn {
    pub fn new(main: &LJK2) -> MidiIn {
        MidiIn {
            main: main.clone(),
            connected_devices: HashSet::new(),
            // connected_devices.insert("Midi Through:Midi Through Port-0 14:0".to_string());
            connections: Vec::new(),
        }
    }

    pub async fn listen(&mut self) {
        loop {
            let mut midi_in = midir::MidiInput::new("LJK2 midi in").unwrap();
            midi_in.ignore(Ignore::None);

            let ports = midi_in.ports();
            for port in ports.iter() {
                let port_name = midi_in.port_name(port).unwrap();
                let device_type = port_name.split(":").next().unwrap();

                if !self.connected_devices.contains(&port_name) {
                    println!("New input device detected: {}", port_name);
                    self.connected_devices.insert(port_name.clone());

                    let callback = match device_type {
                        "Launchpad MK2" => handle_midi_message_launchpad,
                        "X-TOUCH COMPACT" => handle_midi_message_xtouch,
                        _ => {
                            println!("Unknown midi device, not connecting to it.");
                            break;
                        }
                    };

                    let _conn_in: MidiInputConnection<LJK2> = midi_in
                        .connect(port, "LJK2 midi in device", callback, self.main.clone())
                        .unwrap();
                    self.connections.push(_conn_in);
                    break;
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
    }
}

fn handle_midi_message_launchpad<'a, 'b>(_timestamp: u64, message: &'a [u8], data: &'b mut LJK2) {
    let msg = MidiMessage::from(message).unwrap();
    println!("Launchpad: {:?}", msg);
    data.midi.lock().unwrap().send(msg, crate::midi::connection::DeviceType::Launchpad)
}

fn handle_midi_message_xtouch<'a, 'b>(_timestamp: u64, message: &'a [u8], data: &'b mut LJK2) {
    let msg = MidiMessage::from(message).unwrap();
    println!("X-Touch:   {:?}", msg);
    if msg
        == (MidiMessage::NoteOn {
            channel: Channel(0),
            note: Note(93),
            velocity: Velocity(127),
        })
    {
        println!("Stop");
    }
}
