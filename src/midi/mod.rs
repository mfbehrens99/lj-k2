use midi_parse::{Channel, MidiMessage, Note, Velocity};
use midir::{Ignore, MidiInput, MidiInputConnection};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{task, time::sleep};

use crate::LjK2;

pub struct Midi {
    connected_devices: HashSet<String>,
    connections: Vec<MidiInputConnection<()>>
}

impl Midi {
    pub fn new() -> Midi{
        Midi {
            connected_devices: HashSet::new(),
            // connected_devices.insert("Midi Through:Midi Through Port-0 14:0".to_string());
            connections: Vec::new(),
        }
    }

    pub async fn run_midi(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let mut midi_in = MidiInput::new("midir reading input")?;
            midi_in.ignore(Ignore::None);
    
    
            let ports = midi_in.ports();
            for port in ports.iter() {
                let port_name = midi_in.port_name(port)?;
                let device_type = port_name.split(":").next().unwrap();
                if device_type == "Midi Through" {
                    continue;
                }
                if !self.connected_devices.contains(&port_name) {
                    println!("New device detected: {}", port_name);
                    self.connected_devices.insert(port_name.clone());

                    let callback = match device_type {
                        "Launchpad MK2" => handle_midi_message_launchpad,
                        "X-TOUCH COMPACT" => handle_midi_message_xtouch,
                        _ => panic!("Unknown midi device"),
                    };
    
                    let _conn_in: MidiInputConnection<()> = midi_in.connect(
                        port,
                        "midir-read-input",
                        callback,
                        (),
                    ).unwrap();
                    self.connections.push(_conn_in);
                    break;
                }
            }
            // break;
    
            sleep(Duration::from_secs(1)).await;
        }
    }
}

fn handle_midi_message_launchpad<'a, 'b>(timestamp: u64, message: &'a [u8], data: &'b mut ()) {
    let msg = MidiMessage::from(message).unwrap();
    println!("Launchpad: {:?}", msg);
}

fn handle_midi_message_xtouch<'a, 'b>(timestamp: u64, message: &'a [u8], data: &'b mut ()) {
    let msg = MidiMessage::from(message).unwrap();
    println!("X-Touch:   {:?}", msg);
    if msg == (MidiMessage::NoteOn { channel: Channel(0), note: Note(93), velocity: Velocity(127) }) {
        println!("Stop");
    }
}
