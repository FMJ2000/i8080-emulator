use std::{env, fs, thread};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};
pub mod processor;
pub mod machine;
pub mod memory;
pub mod screen;

use machine::{Machine, KeycodeState};
use memory::{ROM_SIZE, VIDEO_SIZE};
use screen::{Resolution, Screen};

pub fn start(mut args: env::Args) {
  args.next();
  if let Some(filename) = args.next() {
    let (tx, rx): (Sender<KeycodeState>, Receiver<KeycodeState>)  = mpsc::channel();
    let screen_video: Arc<Mutex<[u8; VIDEO_SIZE]>> = Arc::new(Mutex::new([0x0; VIDEO_SIZE]));
    let machine_video = Arc::clone(&screen_video);
    let rom: [u8; ROM_SIZE] = fs::read(filename).unwrap()[0..ROM_SIZE].try_into().unwrap();
    let debug = args.next().unwrap_or(String::from("-r"));
    thread::spawn(move || {
      if debug == "-d" {
        Machine::new(rom, machine_video).run_debug(rx);
      } else {
        Machine::new(rom, machine_video).run(rx);
      }
    });
    Screen::new("Space Invaders", Resolution::new(224, 256, 3), screen_video).run(tx);
  }
}