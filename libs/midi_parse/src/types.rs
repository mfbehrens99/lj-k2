use crate::message::MidiMessageError;

macro_rules! MidiType {
    ($type_name:ident, u8, $error_name:ident) => {
        #[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
        pub struct $type_name(pub u8);

        impl $type_name {
            pub fn from(value: u8) -> Result<$type_name, MidiMessageError> {
                match parse_u7(value) {
                    Ok(value) => Ok($type_name(value)),
                    Err(()) => Err(MidiMessageError::$error_name(value)),
                }
            }

            pub fn to_byte(&self) -> u8 {
                self.0
            }
        }
    };
    ($type_name:ident, u16, $error_name:ident) => {
        #[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone)]
        pub struct $type_name(pub u16);

        impl $type_name {
            pub fn from(value1: u8, value2: u8) -> Result<$type_name, MidiMessageError> {
                match parse_u14(value1, value2) {
                    Ok(value) => Ok($type_name(value)),
                    Err(()) => Err(MidiMessageError::$error_name(value1, value2)),
                }
            }
            pub fn to_bytes(&self) -> [u8;2] {
                [(self.0 & 0b0111_1111) as u8, (self.0 >> 7) as u8]
            }
        }
    };
}

MidiType!(Channel, u8, InvalidChannel);
MidiType!(Note, u8, InvalidNote);
MidiType!(Velocity, u8, InvalidVelocity);
MidiType!(Pressure, u8, InvalidPressure);
MidiType!(Controller, u8, InvalidController);
MidiType!(Value, u8, InvalidValue);
MidiType!(Program, u8, InvalidProgram);
MidiType!(Pitch, u16, InvalidPitch);
MidiType!(Position, u16, InvalidPosition);
MidiType!(Song, u8, InvalidSong);

fn parse_u7(value: u8) -> Result<u8, ()> {
    if value & 0b1000_0000 == 0 {
        Ok(value)
    } else {
        Err(())
    }
}

fn parse_u14(pitch2: u8, pitch1: u8) -> Result<u16, ()> {
    // Change bits due to endianess
    if pitch1 & 0b1000_0000 == 0 && pitch2 & 0b1000_0000 == 0 {
        Ok(((pitch1 & 0b0111_1111) as u16) << 7 | ((pitch2 & 0b0111_1111) as u16))
    } else {
        Err(())
    }
}