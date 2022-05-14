use crate::i8080::State;
use std::io;
use std::thread;
use std::time;

pub fn emulate(buffer: Vec<u8>) {
  let mut state = State::new(buffer);
  loop {
    let mut input = String::new();
    state.print();
    //io::stdin().read_line(&mut input).unwrap();
    thread::sleep(time::Duration::from_millis(10));
    state.exec();
  }
}