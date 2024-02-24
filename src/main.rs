mod frontend;
mod midi;

use std::sync::{Arc, Mutex};

use frontend::Frontend;
use midi::Midi;

struct LjK2 {
    pub midi: Midi,
    pub frontend: Frontend,
}

impl LjK2 {
    pub fn new() -> LjK2 {
        LjK2 {
            midi: Midi::new(),
            frontend: Frontend::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut main = LjK2::new();
    // let main_ref = Arc::new(Mutex::new(main));
    tokio::join!(
        main.frontend.listen("127.0.0.1:9002"),
        // run_midi_listener(),
        main.midi.run_midi(),
    );
}
