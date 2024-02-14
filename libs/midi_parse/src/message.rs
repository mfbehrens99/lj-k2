use crate::consts::*;
use crate::types::*;

/// Represents a MIDI message
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum MidiMessage<'a> {
    NoteOn {
        channel: Channel,
        note: Note,
        velocity: Velocity,
    },
    NoteOff {
        channel: Channel,
        note: Note,
        velocity: Velocity,
    },
    PolyphonicKeyPressure {
        channel: Channel,
        note: Note,
        pressure: Pressure,
    },
    ControlChange {
        channel: Channel,
        controller_number: Controller,
        value: Value,
    },
    ProgramChange {
        channel: Channel,
        program_number: Program,
    },
    ChannelPressure {
        channel: Channel,
        pressure: Pressure,
    },
    PitchWheelChange {
        channel: Channel,
        pitch: Pitch,
    },
    SysExMessage(&'a [u8]),
    SongPositionPointer {
        position: Position,
    },
    SongSelect {
        song: Song,
    },
    TuneRequest,
    TimingClock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    Reset,
}

impl<'a> MidiMessage<'a> {
    pub fn from(data: &'a [u8]) -> Result<MidiMessage, MidiMessageError> {
        let length = data.len();
        let channel = Channel::from(data[0] & CHANNEL_MASK)?;
        match data[0] & STATUS_BYTE_MASK {
            NOTE_OFF_MASK => Ok(MidiMessage::NoteOff {
                channel,
                note: Note::from(data[1])?,
                velocity: Velocity::from(data[2])?,
            }),
            NOTE_ON_MASK => Ok(MidiMessage::NoteOn {
                channel,
                note: Note::from(data[1])?,
                velocity: Velocity::from(data[2])?,
            }),
            POLYPHONIC_KEY_PRESSURE_MASK => Ok(MidiMessage::PolyphonicKeyPressure {
                channel,
                note: Note::from(data[1])?,
                pressure: Pressure::from(data[2])?,
            }),
            CONTROL_CHANGE_MASK => {
                // Could potentially detect channel mode change here, but message is the same, the
                // applications can handle this.
                Ok(MidiMessage::ControlChange {
                    channel,
                    controller_number: Controller::from(data[1])?,
                    value: Value::from(data[2])?,
                })
            }
            PROGRAM_CHANGE_MASK => Ok(MidiMessage::ProgramChange {
                channel,
                program_number: Program::from(data[1])?,
            }),
            CHANNEL_PRESSURE_MASK => Ok(MidiMessage::ChannelPressure {
                channel,
                pressure: Pressure::from(data[1])?,
            }),
            PITCH_WHEEL_CHANGE_MASK => Ok(MidiMessage::PitchWheelChange {
                channel,
                pitch: Pitch::from(data[1], data[2])?,
            }),
            SYSEX_MESSAGE_MASK => {
                if data[length - 1] != SYSEX_MESSAGE_END_MASK {
                    return Err(MidiMessageError::InvalidLength);
                }
                let sysex_message: &[u8] = &data[1..=length - 2];
                Ok(MidiMessage::SysExMessage(sysex_message))
            }
            SONG_POSITION_POINTER_MASK => Ok(MidiMessage::SongPositionPointer {
                position: Position::from(data[1], data[2])?,
            }),
            SONG_SELECT_MASK => Ok(MidiMessage::SongSelect {
                song: Song::from(data[1])?,
            }),
            TIMING_CLOCK_MASK => Ok(MidiMessage::TimingClock),
            START_MASK => Ok(MidiMessage::Start),
            CONTINUE_MASK => Ok(MidiMessage::Continue),
            STOP_MASK => Ok(MidiMessage::Stop),
            ACTIVE_SENSING_MASK => Ok(MidiMessage::ActiveSensing),
            RESET_MASK => Ok(MidiMessage::Reset),
            TUNE_REQUEST_MASK => Ok(MidiMessage::TuneRequest),
            _code => Err(MidiMessageError::InvalidCode(_code)),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        use MidiMessage as MM;
        match self {
            MM::NoteOn {
                channel,
                note,
                velocity,
            } => {
                vec![
                    NOTE_ON_MASK | channel.to_byte(),
                    note.to_byte(),
                    velocity.to_byte(),
                ]
            }
            MM::NoteOff {
                channel,
                note,
                velocity,
            } => {
                vec![
                    NOTE_OFF_MASK | channel.to_byte(),
                    note.to_byte(),
                    velocity.to_byte(),
                ]
            }
            MM::PolyphonicKeyPressure {
                channel,
                note,
                pressure,
            } => {
                vec![
                    POLYPHONIC_KEY_PRESSURE_MASK | channel.to_byte(),
                    note.to_byte(),
                    pressure.to_byte(),
                ]
            }
            MM::ControlChange {
                channel,
                controller_number,
                value,
            } => {
                vec![
                    CONTROL_CHANGE_MASK | channel.to_byte(),
                    controller_number.to_byte(),
                    value.to_byte(),
                ]
            }
            MM::ProgramChange {
                channel,
                program_number,
            } => {
                vec![
                    PROGRAM_CHANGE_MASK | channel.to_byte(),
                    program_number.to_byte(),
                ]
            }
            MM::ChannelPressure { channel, pressure } => {
                vec![
                    CHANNEL_PRESSURE_MASK | channel.to_byte(),
                    pressure.to_byte(),
                ]
            }
            MM::PitchWheelChange { channel, pitch } => {
                let [byte2, byte3] = pitch.to_bytes();
                vec![PITCH_WHEEL_CHANGE_MASK | channel.to_byte(), byte2, byte3]
            }
            MM::SysExMessage(message) => [&[SYSEX_MESSAGE_MASK], &message[..]].concat(),
            MM::SongPositionPointer { position } => {
                let [byte2, byte3] = position.to_bytes();
                vec![SONG_POSITION_POINTER_MASK, byte2, byte3]
            }
            MM::SongSelect { song } => {
                vec![SONG_SELECT_MASK, song.to_byte()]
            }
            MM::TimingClock => vec![TIMING_CLOCK_MASK],
            MM::Start => vec![START_MASK],
            MM::Continue => vec![CONTINUE_MASK],
            MM::Stop => vec![STOP_MASK],
            MM::ActiveSensing => vec![ACTIVE_SENSING_MASK],
            MM::Reset => vec![RESET_MASK],
            MM::TuneRequest => vec![TUNE_REQUEST_MASK],
        }
    }

    /// This returns the size in bytes of this message when serialised into MIDI.
    pub fn size_hint(&self) -> usize {
        use MidiMessage as MM;
        match self {
            MM::NoteOn { .. } => 3,
            MM::NoteOff { .. } => 3,
            MM::PolyphonicKeyPressure { .. } => 3,
            MM::ControlChange { .. } => 3,
            MM::ProgramChange { .. } => 2,
            MM::ChannelPressure { .. } => 2,
            MM::PitchWheelChange { .. } => 3,
            MM::SysExMessage(inner) => 2 + inner.len(),
            MM::SongPositionPointer { .. } => 3,
            MM::SongSelect { .. } => 2,
            MM::TuneRequest => 1,
            MM::TimingClock => 1,
            MM::Start => 1,
            MM::Continue => 1,
            MM::Stop => 1,
            MM::ActiveSensing => 1,
            MM::Reset => 1,
        }
    }
}

#[derive(Debug)]
pub enum MidiMessageError {
    InvalidChannel(u8),
    InvalidNote(u8),
    InvalidVelocity(u8),
    InvalidPressure(u8),
    InvalidController(u8),
    InvalidValue(u8),
    InvalidProgram(u8),
    InvalidPitch(u8, u8),
    InvalidPosition(u8, u8),
    InvalidSong(u8),
    InvalidCode(u8),
    InvalidLength,
}
