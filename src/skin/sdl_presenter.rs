extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use std::time::Duration;

use crate::abst::presenter::Presenter;
use crate::exp::string_to_input::StringToInput;

mod header;
mod keyboard;
mod section;
mod text;

use header::Header;
use keyboard::Keyboard;
use section::Section;
use text::TextBuilder;

pub struct SDLPresenter {
  width: u32,
  height: u32,
  ctx: Sdl,
  canvas: Canvas<Window>,
}

impl SDLPresenter {
  pub fn new(
    width: u32,
    height: u32,
  ) -> Result<SDLPresenter, String> {
    let ctx = sdl2::init().map_err(|e| e.to_string())?;

    let video = ctx.video().map_err(|e| e.to_string())?;
    let window = video
      .window("Musical Typer", width, height)
      .position_centered()
      .opengl()
      .build()
      .map_err(|e| e.to_string())?;

    let mut canvas =
      window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.clear();
    canvas.present();

    Ok(SDLPresenter {
      width,
      height,
      ctx,
      canvas,
    })
  }

  fn render<'a, T>(
    &mut self,
    builder: TextBuilder<'a, T>,
  ) -> Result<(), String> {
    let header = Header::new("Music Name", "Composer");
    let header_dim = Rect::new(0, 0, self.width, 100);

    let to_input = StringToInput::new(
      "千本桜　夜ニ紛レ",
      "せんぼんざくらよるにまぎれ",
    )?;
    let section = Section::new(&to_input, 0.2);
    let section_dim = Rect::new(0, 100, self.width, 200);

    let keyboard = Keyboard::new(&['h']);
    let keyboard_dim =
      Rect::new(0, self.height as i32 - 300, self.width, 300);

    self.canvas.set_draw_color(Color::RGB(253, 243, 226));
    self.canvas.clear();

    header.draw(&mut self.canvas, builder.clone())?;
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.draw_rect(header_dim)?;

    section.draw(&mut self.canvas, builder.clone(), section_dim)?;
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.draw_rect(section_dim)?;

    keyboard.draw(&mut self.canvas, builder.clone(), keyboard_dim)?;
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.draw_rect(keyboard_dim)?;
    Ok(())
  }

  pub fn run(&mut self) -> Result<(), String> {
    let texture_creator = self.canvas.texture_creator();

    let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf
      .load_font(
        std::path::Path::new("./asset/mplus-1m-medium.ttf"),
        128,
      )
      .map_err(|e| e.to_string())?;

    let builder = TextBuilder::new(&font, &texture_creator);

    let mut poller =
      self.ctx.event_pump().map_err(|e| e.to_string())?;
    'main: loop {
      for event in poller.poll_iter() {
        use sdl2::event::Event::*;
        match event {
          Quit { .. } => break 'main Ok(()),
          _ => {}
        }
        self.render(builder.clone())?;

        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
      }
    }
  }
}

impl Presenter for SDLPresenter {
  fn play_bgm(&mut self, _: &str) {
    todo!()
  }
  fn decrease_remaining_time(&mut self, _: f64) {
    todo!()
  }
  fn update_string_to_input(&mut self, _: &StringToInput) {
    todo!()
  }
  fn mistyped(&mut self) {
    todo!()
  }
  fn flush_screen(&mut self) {
    todo!()
  }
}
