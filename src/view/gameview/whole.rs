use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};

use super::super::text::TextBuilder;
use super::ViewError;
use crate::model::exp::sentence::Sentence;

mod finder;
mod header;
mod keyboard;

use finder::Finder;
use header::Header;
use keyboard::Keyboard;

pub struct WholeProps<'a> {
  pub pressed_keys: &'a [char],
  pub sentence: &'a Option<Sentence>,
  pub title: &'a str,
  pub song_author: &'a str,
  pub score_point: i32,
}

pub fn render<'a, 't>(
  mut canvas: &mut Canvas<Window>,
  client: Rect,
  builder: TextBuilder<'t, WindowContext>,
  props: &'a WholeProps,
) -> Result<(), ViewError> {
  let header =
    Header::new(props.title, props.song_author, props.score_point);
  let header_dim = Rect::new(0, 0, client.width(), 100);

  let finder = Finder::new(props.sentence, 0.2);
  let finder_dim = Rect::new(0, 100, client.width(), 200);

  let keyboard = Keyboard::new(props.pressed_keys, &[]);
  let keyboard_dim =
    Rect::new(0, client.height() as i32 - 300, client.width(), 300);

  canvas.set_draw_color(Color::RGB(253, 243, 226));
  canvas.clear();

  header.draw(&mut canvas, builder.clone())?;
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas
    .draw_rect(header_dim)
    .map_err(|e| ViewError::RenderError(e))?;

  finder.draw(&mut canvas, builder.clone(), finder_dim)?;
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas
    .draw_rect(finder_dim)
    .map_err(|e| ViewError::RenderError(e))?;

  keyboard.draw(&mut canvas, builder.clone(), keyboard_dim)?;
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas
    .draw_rect(keyboard_dim)
    .map_err(|e| ViewError::RenderError(e))?;
  Ok(())
}
