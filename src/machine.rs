use std::io;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time::Duration;

use crate::processor::Processor;
use crate::memory::{ROM_SIZE, VIDEO_SIZE, VIDEO_START};
//use crate::screen::SCREEN_F;

const CPU_F: u32 = 2_000_000;
const CPU_T: u32 = 1_000_000_000 / CPU_F;
const VBLANK: u32 = CPU_F / 60;

pub struct KeycodeState {
  pub code: Keycode,
  pub pressed: bool,
}

pub struct Machine {
  cpu: Processor,
  video: Arc<Mutex<[u8; VIDEO_SIZE]>>,
  ip: [u8; 8],
  op: [u8; 8],
}

impl Machine {
  pub fn new(rom: [u8; ROM_SIZE], video: Arc<Mutex<[u8; VIDEO_SIZE]>>) -> Machine {
    Machine {
      cpu: Processor::new(rom),
      video,
      ip: [0; 8],
      op: [0; 8],
    }
  }

  pub fn run(&mut self, rx: Receiver<KeycodeState>) {
    let mut counter: usize = 0;
    let mut video_counter: u32 = VBLANK;
    let mut screen_end = false;
    loop {
      if counter <= 0 {
        if let Ok(keycode) = rx.try_recv() {
          self.key_state_change(keycode);
        }
        counter = self.exec();
      } else {
        counter -= 1;
      }
      if video_counter <= 0 {
        self.map_video(screen_end);
        screen_end ^= true;
        video_counter = VBLANK;
      } else {
        video_counter -= 1;
      }
      thread::sleep(Duration::new(0, CPU_T));
    }
  }

  pub fn run_debug(&mut self, rx: Receiver<KeycodeState>) {
    let mut counter: usize = 0;
    let mut video_counter: u32 = VBLANK;
    let mut input_counter: Option<usize> = None;
    let mut screen_end = false;
    loop {
      if counter <= 0 {
        if let Ok(keycode) = rx.try_recv() {
          self.key_state_change(keycode);
        }
        counter = self.exec();
        if input_counter == None || input_counter == Some(self.cpu.pc) {
          self.print();
          let mut input = String::new();
          io::stdin().read_line(&mut input).unwrap();
          input_counter = match usize::from_str_radix(input.trim(), 16) {
            Ok(x) => Some(x),
            _ => None,
          };
        }
      } else {
        counter -= 1;
      }
      if video_counter <= 0 {
        self.map_video(screen_end);
        screen_end ^= true;
        video_counter = VBLANK;
      } else {
        video_counter -= 1;
      }
    }
  }
  
  pub fn exec(&mut self) -> usize {
    let opcode = self.cpu.mem.read(self.cpu.pc);
    let port = self.cpu.mem.read(self.cpu.pc+1);

    match opcode {
      0xDB => {
        self.cpu.a = self.input(port);
        self.cpu.pc += 2;
        10
      },
      0xD3 => {
        self.output(port, self.cpu.a);
        self.cpu.pc += 2;
        10
      },
      _ => self.cpu.exec(),
    }
  }

  fn map_video(&mut self, screen_end: bool) {
    self.cpu.int(if screen_end { 2 } else { 1 });
    /*if self.cpu.ie {
      println!("Interrupt");
      self.cpu.print();
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
    }*/
    if let Ok(ref mut video) = self.video.try_lock() {
      for i in 0..VIDEO_SIZE {
          video[i] = self.cpu.mem.read(VIDEO_START+i);
      }
    }
  }

  pub fn print(&self) {
    self.cpu.print();
    println!("IP:\t{:?}", self.ip);
    println!("OP:\t{:?}", self.op);
  }

  fn input(&mut self, port: u8) -> u8 {
    match port {
      0x3 => ((((self.op[4] as u16) << 8) | self.ip[3] as u16) >> (8 - self.op[2])) as u8,
      _ => self.cpu.a,
    }
  }

  fn output(&mut self, port: u8, db: u8) {
    match port {
      0x2 => self.op[2] = db & 0x7,
      0x4 => {
        self.ip[3] = self.op[4];
        self.op[4] = db;
      },
      _ => (),
    }
  }

  fn key_state_change(&mut self, state: KeycodeState) {
    if state.pressed {
      match state.code {
        Keycode::C => self.ip[1] |= 0x01,
        Keycode::X => self.ip[1] |= 0x02,
        Keycode::Z => self.ip[1] |= 0x04,
        Keycode::Space => self.ip[1] |= 0x10,
        Keycode::Left => self.ip[1] |= 0x20,
        Keycode::Right => self.ip[1] |= 0x40,
        _ => (),
      }
    } else {
      match state.code {
        Keycode::C => self.ip[1] &= !0x01,
        Keycode::X => self.ip[1] &= !0x02,
        Keycode::Z => self.ip[1] &= !0x04,
        Keycode::Space => self.ip[1] &= !0x10,
        Keycode::Left => self.ip[1] &= !0x20,
        Keycode::Right => self.ip[1] &= !0x40,
        _ => (),
      }
    }
    self.cpu.print();
  }
}