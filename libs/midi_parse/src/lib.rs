mod consts;
mod message;
mod types;

pub use message::{MidiMessage, MidiMessageError};
pub use types::*;

// TODO: Write tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_off() {
        let data = vec![0b1000_0000, 0, 0];
        let result = MidiMessage::from(&data).unwrap();
        assert_eq!(
            result,
            MidiMessage::NoteOff {
                channel: Channel(0),
                note: Note(0),
                velocity: Velocity(0)
            }
        );

        let data_check = result.to_bytes();
        assert_eq!(data, data_check);
    }

    #[test]
    fn test_note_on() {
        let data = vec![0b1001_0000, 69, 127];
        let result = MidiMessage::from(&data).unwrap();
        assert_eq!(
            result,
            MidiMessage::NoteOn {
                channel: Channel(0),
                note: Note(69),
                velocity: Velocity(127)
            }
        );

        let data_check = result.to_bytes();
        assert_eq!(data, data_check);
    }

    #[test]
    fn test_pitch() {
        let data = vec![0b1110_0100, 0, 96];
        let result = MidiMessage::from(&data).unwrap();
        assert_eq!(
            result,
            MidiMessage::PitchWheelChange {
                channel: Channel(4),
                pitch: Pitch(12288),
            }
        );

        let data_check = result.to_bytes();
        assert_eq!(data, data_check);
    }

    #[test]
    fn test_sys_ex() {
        let data = vec![0b1111_0000, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0b1111_0111];
        let result = MidiMessage::from(&data).unwrap();
        let midi_data = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(result, MidiMessage::SysExMessage(Box::new(midi_data)));

        let data_check = result.to_bytes();
        assert_eq!(data, data_check);
    }
}
