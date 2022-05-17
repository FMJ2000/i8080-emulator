use std::{env, fs, thread};
use std::sync::mpsc::{self, Sender, Receiver};
pub mod processor;
pub mod machine;
pub mod memory;
pub mod screen;

use machine::{Machine, KeycodeState};
use memory::ROM_SIZE;
use screen::{Resolution, Screen};

pub fn start(mut args: env::Args) {
  args.next();
  if let Some(filename) = args.next() {
    let (tx, rx): (Sender<KeycodeState>, Receiver<KeycodeState>)  = mpsc::channel();
    let rom: [u8; ROM_SIZE] = fs::read(filename).unwrap()[0..ROM_SIZE].try_into().unwrap();
    thread::spawn(move || {
      Machine::new(rom).run(rx);
    });
    Screen::new("Space Invaders", Resolution::new(256, 224, 3)).run(tx);
  }
}