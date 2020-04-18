use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};

use super::super::{stats, stats::StatsProps};
use super::ViewError;
use crate::{model::exp::sentence::Sentence, view::text::TextCtx};

mod finder;
mod header;
mod keyboard;

use finder::FinderProps;
use header::HeaderProps;
use keyboard::KeyboardProps;

pub struct WholeProps {
  pub pressed_keys: Vec<char>,
  pub sentence: Option<Sentence>,
  pub title: String,
  pub song_author: String,
  pub score_point: i32,
  pub type_per_second: f64,
  pub achievement_rate: f64,
  pub accuracy: f64,
}

pub fn build(
  client: Rect,
  builder: TextCtx<'_, WindowContext>,
  props: WholeProps,
) -> Result<
  impl Fn(&mut Canvas<Window>) -> Result<(), ViewError> + '_,
  ViewError,
> {
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

  let keyboard_dim = Rect::new(0, 250, client.width(), 200);
  let keyboard_render = keyboard::build(
    builder.clone(),
    keyboard_dim,
    KeyboardProps {
      pressed_keys: props.pressed_keys.clone(),
      highlighted_keys: vec![],
    },
  )?;

  let stats_dim =
    Rect::new(0, 450, client.width(), client.height() - 450);
  let stats_render = stats::build(
    stats_dim,
    builder.clone(),
    StatsProps {
      accuracy: props.accuracy,
      type_per_second: props.type_per_second,
      achievement_rate: props.achievement_rate,
    },
  )?;

  Ok(move |mut canvas: &mut Canvas<Window>| {
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

    keyboard_render(&mut canvas)?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas
      .draw_rect(keyboard_dim)
      .map_err(|e| ViewError::RenderError(e))?;

    stats_render(&mut canvas)?;
    Ok(())
  })
}
