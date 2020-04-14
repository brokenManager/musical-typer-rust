use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};

use super::text::TextBuilder;
use super::ViewError;

mod rank;

pub struct StatsProps {
  pub type_per_second: f64,
  pub achievement_rate: f64,
  pub accuracy: f64,
}

pub fn render<'a, 't>(
  mut canvas: &mut Canvas<Window>,
  client: Rect,
  builder: TextBuilder<'t, WindowContext>,
  props: &'a StatsProps,
) -> Result<(), ViewError> {
  let rank = rank::rank(props.accuracy);
  Ok(())
}
