mod frontend;
mod midi;
mod resolume;

use grandma2::GrandMa2;

use midi_parse::{Channel, MidiMessage, Note, Velocity};
use tokio::sync::mpsc;

use crate::{
    frontend::SendMessage,
    midi::{DeviceType, MessageMidi},
};

#[tokio::main]
async fn main() {
    let (frontend_tx, mut frontend_rx) = mpsc::channel(100);
    let mut frontend = frontend::Server::new("0.0.0.0:9002", frontend_tx);
    let frontend_tx = frontend.get_sender();

    let (midi_tx, mut midi_rx) = mpsc::channel(100);
    let mut midi_in = midi::MidiIn::new(midi_tx);

    let mut midi_out = midi::MidiOut::new();
    let midi_tx = midi_out.get_sender();

    let mut grandma = GrandMa2::new("ws://10.1.1.10", "remote", "remote");
    grandma.connect();

    tokio::task::spawn(async move {
        loop {
            tokio::select! {
                // Receive message from Interface
                msg = midi_rx.recv() => {
                    if let Some(msg) = msg {
                        midi_tx.send(msg.clone()).await.unwrap();
                        if let MidiMessage::NoteOn{channel: _, note, velocity: _} = msg.midi_msg {
                            frontend_tx.send(SendMessage::SendFaderState{row: 0, column: 1, value: (note.0 as f32 / 127.0)}).await.unwrap();
                        }
                    }
                }
                msg = frontend_rx.recv() => {
                    if let Some(msg) = msg {
                        match msg {
                            frontend::ReceiveMessage::SetPreset{row: 0, column} => {
                                    midi_tx.send(MessageMidi { midi_msg: MidiMessage::NoteOn { channel: Channel(0), note: Note(11 + column), velocity: Velocity(127) }, device_type: DeviceType::Launchpad}).await.unwrap();
                            }
                            frontend::ReceiveMessage::SetPreset{row: 1, column} => {
                                    midi_tx.send(MessageMidi { midi_msg: MidiMessage::NoteOn { channel: Channel(0), note: Note(11 + column), velocity: Velocity(0) }, device_type: DeviceType::Launchpad}).await.unwrap();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    });

    tokio::join!(frontend.run(), midi_in.run(), midi_out.run(), grandma.run(),);
}
