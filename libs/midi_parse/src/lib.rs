mod consts;
mod message;
mod types;

pub use message::{MidiMessage, MidiMessageError};
pub use types::*;

// TODO: Write tests
#[cfg(test)]
mod tests;