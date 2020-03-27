extern crate sdl2;

use sdl2::pixels::Color;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
  let ctx = sdl2::init().unwrap();
  let video = ctx.video().unwrap();
  let window = video
    .window("Musical Typer", WIDTH, HEIGHT)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();
  canvas.set_draw_color(Color::RGB(0, 255, 0));
  canvas.clear();
  canvas.present();

  let mut poller = ctx.event_pump().unwrap();

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
    canvas.clear();
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
  }
}
