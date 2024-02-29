mod frontend;
mod midi;

use std::sync::{Arc, Mutex};

use frontend::Frontend;
use midi::{MidiIn, MidiInterface, MidiOut};

#[derive(Clone)]
struct LJK2 {
    pub midi: Arc<Mutex<MidiInterface>>,
    // pub frontend: Arc<Mutex<Frontend>>,
    // pub grandma: Arc<Mutex<GrandMa2>>.
}

impl LJK2 {
    pub fn new() -> LJK2 {
        LJK2 {
            midi: Arc::new(Mutex::new(MidiInterface::new())),
            // frontend: Arc::new(Mutex::new(Frontend))
            // grandma: GrandMa2::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    let main = LJK2::new();
    let mut frontend = Frontend::new(&main, "127.0.0.1:9002");
    let mut midi_in = MidiIn::new(&main);
    let mut midi_out= MidiOut::new(&main);

    tokio::join!(
        frontend.listen(),
        midi_in.listen(),
        midi_out.listen(),
        // main.grandma.listen(),
    );
}
