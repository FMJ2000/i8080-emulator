use sdl2::rect::Rect;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, render::Canvas, video::Window, EventPump};
use std::sync::{Mutex, Arc};
use std::sync::mpsc::Sender;
use std::{thread, time::Duration};

use crate::machine::KeycodeState;
use crate::memory::VIDEO_SIZE;

pub const SCREEN_F: u32 = 60;
const SCREEN_T: u32 = 1_000_000_000 / SCREEN_F;

pub struct Resolution {
  width: usize,
  height: usize,
  scale: usize,
}

impl Resolution {
  pub fn new(width: usize, height: usize, scale: usize) -> Resolution {
    Resolution { width, height, scale }
  }
}

pub struct Screen {
  canvas: Canvas<Window>,
  res: Resolution,
  events: EventPump,
  video: Arc<Mutex<[u8; VIDEO_SIZE]>>
}

impl Screen {
  pub fn new(title: &'static str, res: Resolution, video: Arc<Mutex<[u8; VIDEO_SIZE]>>) -> Screen {
    let context = sdl2::init().unwrap();
    let video_context = context.video().unwrap();
    let window = video_context.window(title, (res.width * res.scale) as u32, (res.height * res.scale) as u32).build().unwrap();

    Screen {
      canvas: window.into_canvas().build().unwrap(),
      events: context.event_pump().unwrap(), 
      res,
      video,
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
          Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::Space, pressed: true }).unwrap(); },
          Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::Left, pressed: false }).unwrap(); },
          Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::Right, pressed: false }).unwrap(); },
          Event::KeyUp { keycode: Some(Keycode::Z), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::Z, pressed: false }).unwrap(); },
          Event::KeyUp { keycode: Some(Keycode::X), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::X, pressed: false }).unwrap(); },
          Event::KeyUp { keycode: Some(Keycode::C), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::C, pressed: false }).unwrap(); },
          Event::KeyUp { keycode: Some(Keycode::Space), repeat: false, .. } => { tx.send(KeycodeState { code: Keycode::Space, pressed: false }).unwrap(); },
          _ => (),
        }
      }
      self.draw();
      thread::sleep(Duration::new(0, SCREEN_T));
    }
  }

  fn draw(&mut self) {
    let scale = self.res.scale as u32;
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.clear();
    let mut cloned_video: Option<[u8; VIDEO_SIZE]> = None;
    {
      let video = self.video.lock().unwrap();
      cloned_video = Some(video.clone());
    }
    if let Some(video) = cloned_video {
      for i in 0..VIDEO_SIZE {
        let byte = video[i];
        for j in 0..8 {
          if (byte >> j) & 0x1 == 1 {
            self.canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
            let y = (self.res.height - ((i % 0x20) * 8 + j)) as i32;
            let x = (i / 0x20) as i32;
            self.canvas.fill_rect(Rect::new(x*(scale as i32), y*(scale as i32), scale, scale)).unwrap();
          }
        }
      }
    }
    self.canvas.present();
  }
  //self.canvas.set_draw_color(Color::RGB(0x0, 0xFF, 0x0));
  //self.canvas.fill_rect(Rect::new(30, 30, self.res.scale, self.res.scale)).unwrap();
}