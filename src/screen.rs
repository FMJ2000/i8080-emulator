use sdl2::rect::Rect;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, render::Canvas, video::Window, EventPump};
use std::sync::mpsc::Sender;
use std::{thread, time::Duration};

use crate::machine::KeycodeState;

pub struct Resolution {
  width: u32,
  height: u32,
  scale: u32,
}

impl Resolution {
  pub fn new(width: u32, height: u32, scale: u32) -> Resolution {
    Resolution { width, height, scale }
  }
}

pub struct Screen {
  canvas: Canvas<Window>,
  res: Resolution,
  events: EventPump
}

impl Screen {
  pub fn new(title: &'static str, res: Resolution) -> Screen {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video.window(title, (res.width * res.scale) as u32, (res.height * res.scale) as u32).build().unwrap();

    Screen {
      canvas: window.into_canvas().build().unwrap(),
      events: context.event_pump().unwrap(), 
      res,
    }
  }

  pub fn run(&mut self, tx: Sender<KeycodeState>) {
    'running: loop {
      for event in self.events.poll_iter() {
        match event {
          Event::Quit {..} |
          Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running; },
          Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::Left, pressed: true }).unwrap(); },
          Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::Right, pressed: true }).unwrap(); },
          Event::KeyDown { keycode: Some(Keycode::Z), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::Z, pressed: true }).unwrap(); },
          Event::KeyDown { keycode: Some(Keycode::X), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::X, pressed: true }).unwrap(); },
          Event::KeyDown { keycode: Some(Keycode::C), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::C, pressed: true }).unwrap(); },
          Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::Left, pressed: false }).unwrap(); },
          Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::Right, pressed: false }).unwrap(); },
          Event::KeyUp { keycode: Some(Keycode::Z), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::Z, pressed: false }).unwrap(); },
          Event::KeyUp { keycode: Some(Keycode::X), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::X, pressed: false }).unwrap(); },
          Event::KeyUp { keycode: Some(Keycode::C), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::C, pressed: false }).unwrap(); },
          _ => (),
        }
      }
      self.draw();
      thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
  }

  fn draw(&mut self) {
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.clear();
    /*for i in 0..SCREEN_RES.0 {
      for j in 0..SCREEN_RES.1 {
        let byte = self.machine.cpu.mem.read(RAM_VIDEO+ i*SCREEN_RES.0 + j);
        for k in 0..8 {
          if (byte >> k) & 0x1 == 1 {
            self.canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
            self.canvas.fill_rect(Rect::new(i as i32, j as i32, SCREEN_RES.2 as u32, SCREEN_RES.2 as u32)).unwrap();
          }
        }
      }
    }*/
    self.canvas.set_draw_color(Color::RGB(0x0, 0xFF, 0x0));
    self.canvas.fill_rect(Rect::new(30, 30, self.res.scale, self.res.scale)).unwrap();
    self.canvas.present();
  }
}