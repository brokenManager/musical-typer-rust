use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use super::super::renderer::{
  text::TextAlign, RenderCtx, ViewResult,
};
use crate::{
  model::exp::game_activity::GameScore, view::renderer::Component,
};

mod rank;

#[derive(PartialEq)]
pub struct StatsProps {
  pub type_per_second: f64,
  pub score: GameScore,
}

pub struct Stats {
  props: StatsProps,
  client: Rect,
}

impl Stats {
  pub fn new(props: StatsProps, client: Rect) -> Self {
    Self { props, client }
  }
}

impl Component for Stats {
  type Props = StatsProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.props == new_props
  }

  fn update(&mut self, new_props: Self::Props) {
    self.props = new_props;
  }

  fn render(&self, ctx: RenderCtx<'_, '_>) -> ViewResult {
    let &Stats { props, client } = &self;
    let &StatsProps {
      type_per_second,
      score,
    } = &props;

    let accuracy = score.accuracy;
    let achievement_rate = score.achievement_rate;

    let speed_indicator_color = if 4.0 < *type_per_second {
      Color::RGB(250, 119, 109)
    } else {
      Color::RGB(178, 255, 89)
    };

    let rank = rank::rank(accuracy * 200.0);

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
