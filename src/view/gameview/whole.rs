use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};

use super::super::text::TextBuilder;
use super::super::{stats, stats::StatsProps};
use super::ViewError;
use crate::model::exp::sentence::Sentence;

mod finder;
mod header;
mod keyboard;

use finder::FinderProps;
use header::HeaderProps;
use keyboard::Keyboard;

pub struct WholeProps<'a> {
  pub pressed_keys: &'a [char],
  pub sentence: Option<Sentence>,
  pub title: String,
  pub song_author: String,
  pub score_point: i32,
  pub type_per_second: f64,
  pub achievement_rate: f64,
  pub accuracy: f64,
}

pub fn render<'a, 't>(
  mut canvas: &mut Canvas<Window>,
  client: Rect,
  builder: TextBuilder<'t, WindowContext>,
  props: &'a WholeProps,
) -> Result<(), ViewError> {
  let header_render = header::build(
    builder.clone(),
    HeaderProps {
      title: props.title.clone(),
      author: props.song_author.clone(),
      score_point: props.score_point,
    },
  )?;
  let header_dim = Rect::new(0, 0, client.width(), 100);

  let finder_dim = Rect::new(0, 100, client.width(), 150);
  let finder_render = finder::build(
    builder.clone(),
    finder_dim,
    FinderProps {
      remaining_ratio: 0.2,
      sentence: props.sentence.clone(),
    },
  )?;

  let keyboard = Keyboard::new(props.pressed_keys, &[]);
  let keyboard_dim = Rect::new(0, 250, client.width(), 200);

  let stats_dim =
    Rect::new(0, 450, client.width(), client.height() - 450);

  canvas.set_draw_color(Color::RGB(253, 243, 226));
  canvas.clear();

  header_render(&mut canvas)?;
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas
    .draw_rect(header_dim)
    .map_err(|e| ViewError::RenderError(e))?;

  finder_render(&mut canvas)?;
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas
    .draw_rect(finder_dim)
    .map_err(|e| ViewError::RenderError(e))?;

  keyboard.draw(&mut canvas, builder.clone(), keyboard_dim)?;
  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas
    .draw_rect(keyboard_dim)
    .map_err(|e| ViewError::RenderError(e))?;

  stats::build(
    stats_dim,
    builder.clone(),
    StatsProps {
      accuracy: props.accuracy,
      type_per_second: props.type_per_second,
      achievement_rate: props.achievement_rate,
    },
  )?(&mut canvas)?;
  Ok(())
}
