use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};

use super::text::TextBuilder;
use super::ViewError;
use crate::model::exp::sentence::Sentence;

mod header;
mod keyboard;
mod section;

use header::Header;
use keyboard::Keyboard;
use section::Section;

pub fn render<'t>(
  mut canvas: &mut Canvas<Window>,
  client: Rect,
  builder: TextBuilder<'t, WindowContext>,
) -> Result<(), ViewError> {
  let header = Header::new("Music Name", "Composer");
  let header_dim = Rect::new(0, 0, client.width(), 100);

  let to_input =
    Sentence::new("千本桜　夜ニ紛レ", "せんぼんざくらよるにまぎれ")
      .map_err(|e| ViewError::InitError {
      message: format!("{:?}", e),
    })?;
  let section = Section::new(&to_input, 0.2);
  let section_dim = Rect::new(0, 100, client.width(), 200);

  let keyboard = Keyboard::new(&['h']);
  let keyboard_dim =
    Rect::new(0, client.height() as i32 - 300, client.width(), 300);

  canvas.set_draw_color(Color::RGB(253, 243, 226));
  canvas.clear();

  header.draw(&mut canvas, builder.clone())?;
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas
    .draw_rect(header_dim)
    .map_err(|e| ViewError::RenderError(e))?;

  section.draw(&mut canvas, builder.clone(), section_dim)?;
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas
    .draw_rect(section_dim)
    .map_err(|e| ViewError::RenderError(e))?;

  keyboard.draw(&mut canvas, builder.clone(), keyboard_dim)?;
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas
    .draw_rect(keyboard_dim)
    .map_err(|e| ViewError::RenderError(e))?;
  Ok(())
}
