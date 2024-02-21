use midi_parse::{MidiMessage, Note, Velocity, Channel};
use midir::{Ignore, MidiInput};
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    let in_port_name = midi_in.port_name(in_port)?;

    // let mut state = ParserState::default();

    println!("Connecting to '{}'", in_port_name);

    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_stamp, data, _| {
            let msg = match MidiMessage::from(data) {
                Ok(msg) => {msg}
                Err(err) => {
                    println!("Error: {:?}", err);
                    return;
                }
            };

            // println!("Midi message: {:?}", msg);

            match msg {
                MidiMessage::NoteOn {
                    channel: _chan,
                    note: Note(note  @ 11..=89),
                    velocity: Velocity(vel),
                } => {
                    println!("Noteon note {:?}: vel: {:?}", note, vel);
                }
                MidiMessage::PitchWheelChange {
                    channel: Channel(chan @ 0..=7),
                    pitch: value,
                } => {
                    println!("Pitch on chan {:?}: vel: {:?}", chan, value);
                }
                _ => {}
            }
        },
        (),
    )?;

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    Ok(())

}

