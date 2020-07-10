use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use super::super::renderer::{
  text::TextAlign, RenderCtx, ViewResult,
};
use crate::model::exp::game_activity::GameScore;

mod rank;

pub fn stats(
  type_per_second: f64,
  score: GameScore,
) -> impl Fn(RenderCtx, Rect) -> ViewResult {
  let accuracy = score.accuracy;
  let achievement_rate = score.achievement_rate;

  let speed_indicator_color = if 4.0 < type_per_second {
    Color::RGB(250, 119, 109)
  } else {
    Color::RGB(178, 255, 89)
  };

  let rank = rank::rank(accuracy * 200.0);

  move |ctx: RenderCtx, client: Rect| {
    let mut canvas = ctx.borrow_mut();

    let speed_indicator_center =
      Point::new(client.width() as i32 / 2, client.y() + 15);
    canvas.set_draw_color(speed_indicator_color);
    canvas.fill_rect(Rect::from_center(
      speed_indicator_center,
      client.width() - 20,
      20,
    ))?;

    canvas.text(|s| {
      s.text(&format!("{:04.2} Type/s", type_per_second))
        .color(Color::RGB(0, 0, 0))
        .line_height(20)
        .align(TextAlign::Center)
        .pos(speed_indicator_center)
    })?;

    canvas.text(|s| {
      s.text("正解率")
        .color(Color::RGB(160, 160, 165))
        .line_height(20)
        .pos(client.top_left().clone().offset(10, 30))
    })?;
    canvas.text(|s| {
      s.text(&format!("{:05.1}%", accuracy * 100.0))
        .color(Color::RGB(
          (250.0 * accuracy) as u8,
          (120.0 * accuracy) as u8,
          (110.0 * accuracy) as u8,
        ))
        .line_height(client.height() - 20)
        .pos(client.top_left().clone().offset(10, 30))
    })?;
    canvas.set_draw_color(Color::RGB(250, 120, 110));
    canvas.draw_rect(Rect::new(
      client.left() + 10,
      client.bottom() - 10,
      (client.width() as f64 * 0.5 * accuracy) as u32,
      2,
    ))?;

    canvas.text(|s| {
      s.text("達成率")
        .color(Color::RGB(160, 160, 165))
        .line_height(20)
        .pos(Point::new(
          client.width() as i32 / 2 + client.x() + 10,
          client.y() + 30,
        ))
    })?;
    canvas.text(|s| {
      s.text(&format!("{:05.1}%", achievement_rate * 100.0))
        .color(Color::RGB(64, 79, 181))
        .line_height(client.height() - 20)
        .pos(Point::new(
          client.width() as i32 / 2 + client.x() + 10,
          client.y() + 30,
        ))
    })?;

    canvas.text(|s| {
      s.text("ランク")
        .color(Color::RGB(160, 160, 165))
        .pos(client.top_left().clone().offset(10, -40))
    })?;
    canvas.text(|s| {
      s.text(rank)
        .color(Color::RGB(64, 79, 181))
        .line_height(25)
        .pos(client.top_left().clone().offset(10, -25))
    })?;
    Ok(())
  }
}
