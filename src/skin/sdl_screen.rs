extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;

use crate::abst::screen::Screen;
use crate::exp::string_to_input::StringToInput;

pub struct SDLScreen {
  ctx: Sdl,
  canvas: Canvas<Window>,
}

impl SDLScreen {
  pub fn new(width: u32, height: u32) -> SDLScreen {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();
    let window = video
      .window("Musical Typer", width, height)
      .position_centered()
      .opengl()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(253, 243, 226));
    canvas.clear();
    canvas.present();

    SDLScreen { ctx, canvas }
  }

  pub fn run(&mut self) {
    let mut poller = self.ctx.event_pump().unwrap();

    'main: loop {
      for event in poller.poll_iter() {
        use sdl2::event::Event::*;
        match event {
          Quit { .. } => {
            break 'main;
          }
          _ => {}
        }
      }
      self.canvas.clear();
      self.canvas.present();
      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
  }
}

impl Screen for SDLScreen {
  fn play_bgm(_: &str) {
    todo!()
  }
  fn decrease_remaining_time(_: f64) {
    todo!()
  }
  fn update_string_to_input(_: &StringToInput) {
    todo!()
  }
}
