use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
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
  let speed_indicator_color = if 4.0 < props.type_per_second {
    Color::RGB(250, 119, 109)
  } else {
    Color::RGB(178, 255, 89)
  };
  let speed_indicator_center =
    Point::new(client.width() as i32 / 2, client.y() + 15);
  canvas.set_draw_color(speed_indicator_color);
  canvas
    .fill_rect(Rect::from_center(
      speed_indicator_center,
      client.width() - 20,
      20,
    ))
    .map_err(|e| ViewError::RenderError(e))?;
  builder
    .text(&format!("{:04.2} Type/s", props.type_per_second))
    .color(Color::RGB(0, 0, 0))
    .build()?
    .render(
      &mut canvas,
      Rect::from_center(speed_indicator_center, 100, 20),
    )?;
  builder
    .text("正解率")
    .color(Color::RGB(160, 160, 165))
    .build()?
    .render(
      &mut canvas,
      Rect::new(client.x() + 10, client.y() + 30, 50, 20),
    )?;
  builder
    .text(&format!("{:05.1}%", props.accuracy))
    .color(Color::RGB(0, 0, 0))
    .build()?
    .render(
      &mut canvas,
      Rect::new(
        client.x() + 60,
        client.y() + 30,
        ((client.width() / 2) as i32 - (client.x() + 60)) as u32,
        client.height() - 20,
      ),
    )?;
  builder
    .text("達成率")
    .color(Color::RGB(160, 160, 165))
    .build()?
    .render(
      &mut canvas,
      Rect::new(
        client.width() as i32 / 2 + client.x() + 10,
        client.y() + 30,
        50,
        20,
      ),
    )?;
  builder
    .text(&format!("{:05.1}%", props.achievement_rate))
    .color(Color::RGB(64, 79, 181))
    .build()?
    .render(
      &mut canvas,
      Rect::new(
        (client.width() / 2) as i32 + client.x() + 60,
        client.y() + 30,
        ((client.width() / 2) as i32 - (client.x() + 60)) as u32,
        client.height() - 20,
      ),
    )?;
  let rank = rank::rank(props.accuracy);
  Ok(())
}
