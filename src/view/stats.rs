use sdl2::pixels::Color;
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

pub fn render<'t>(
  mut canvas: &mut Canvas<Window>,
  client: Rect,
  mut builder: TextBuilder<'t, WindowContext>,
  props: StatsProps,
) -> Result<(), ViewError> {
  builder
    .text("タイピング速度")
    .color(Color::RGB(160, 160, 165))
    .build()?
    .render(
      &mut canvas,
      Rect::new(client.x() + 10, client.y() + 10, 70, 10),
    )?;
  builder.text("正解率").build()?.render(
    &mut canvas,
    Rect::new(client.x() + 10, client.y() + 30, 30, 10),
  )?;
  builder.text("達成率").build()?.render(
    &mut canvas,
    Rect::new(
      client.width() as i32 / 2 + client.x() + 10,
      client.y() + 30,
      30,
      10,
    ),
  )?;
  let rank = rank::rank(props.accuracy);
  Ok(())
}
