// The first 4 bits of the status byte indicate message type. This bitmask extracts that
// section to match against the masks below.
/// bit-mask to match the status byte
pub const STATUS_BYTE_MASK: u8 = 0b1111_0000;
pub const CHANNEL_MASK: u8 = 0b0000_1111;

// Bit-masks for each of the statuses, the 2nd 4 bits indicate the MIDI channel
pub const CONTROL_CHANGE_MASK: u8 = 0b1011_0000;
pub const NOTE_OFF_MASK: u8 = 0b1000_0000;
pub const NOTE_ON_MASK: u8 = 0b1001_0000;
pub const POLYPHONIC_KEY_PRESSURE_MASK: u8 = 0b1010_0000;
pub const PROGRAM_CHANGE_MASK: u8 = 0b1100_0000;
pub const CHANNEL_PRESSURE_MASK: u8 = 0b1101_0000;
pub const PITCH_WHEEL_CHANGE_MASK: u8 = 0b1110_0000;

// All these messages start with 0b1111, the 2nd 4 bits are part of the status
pub const SONG_POSITION_POINTER_MASK: u8 = 0b1111_0010;
pub const SONG_SELECT_MASK: u8 = 0b1111_0011;
pub const TIMING_CLOCK_MASK: u8 = 0b1111_1000;
pub const START_MASK: u8 = 0b1111_1010;
pub const CONTINUE_MASK: u8 = 0b1111_1011;
pub const STOP_MASK: u8 = 0b1111_1100;
pub const ACTIVE_SENSING_MASK: u8 = 0b1111_1110;
pub const RESET_MASK: u8 = 0b1111_1111;
pub const TUNE_REQUEST_MASK: u8 = 0b1111_0110;

pub const SYSEX_MESSAGE_MASK: u8 = 0b1111_0000;
pub const SYSEX_MESSAGE_END_MASK: u8 = 0b11110111;
