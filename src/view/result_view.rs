use super::{
  components::{header, stats},
  handler::Handler,
  renderer::RenderCtx,
  View, ViewRoute,
};
use crate::model::exp::{
  game_activity::GameScore, scoremap::MusicInfo,
};
use sdl2::{
  pixels::Color,
  rect::{Point, Rect},
};
use std::time::Instant;

pub struct ResultView<'ttf, 'canvas> {
  renderer: RenderCtx<'ttf, 'canvas>,
  handler: Handler,
  score: GameScore,
  music_info: MusicInfo,
}

impl<'ttf, 'canvas> ResultView<'ttf, 'canvas> {
  pub fn new(
    renderer: RenderCtx<'ttf, 'canvas>,
    handler: Handler,
    score: GameScore,
    music_info: MusicInfo,
  ) -> Self {
    Self {
      renderer,
      handler,
      score,
      music_info,
    }
  }
}

impl<'ttf, 'canvas> View for ResultView<'ttf, 'canvas> {
  fn run(&mut self) -> Result<ViewRoute, super::ViewError> {
    let client = Rect::new(
      0,
      0,
      self.renderer.borrow().width(),
      self.renderer.borrow().height(),
    );

    let mut mouse_pos = Point::new(0, 0);
    let mut mouse_pressed = false;
    let mut started_pressing = Point::new(0, 0);
    let mut ended_pressing = Point::new(0, 0);

    'main: loop {
      let time = Instant::now();
      {
        use sdl2::event::Event::*;
        let mut should_quit = false;
        self.handler.poll_events(|event| match event {
          Quit { .. } => {
            should_quit = true;
          }
          KeyDown { .. } => {
            should_quit = true;
          }
          MouseMotion { x, y, .. } => {
            mouse_pos = Point::new(x, y);
          }
          MouseButtonDown {
            x, y, mouse_btn, ..
          } => {
            use sdl2::mouse::MouseButton::*;
            if let Left = mouse_btn {
              mouse_pressed = true;
              started_pressing = Point::new(x, y);
              ended_pressing = Point::new(0, 0);
            }
          }
          MouseButtonUp {
            x, y, mouse_btn, ..
          } => {
            use sdl2::mouse::MouseButton::*;
            if let Left = mouse_btn {
              mouse_pressed = false;
              ended_pressing = Point::new(x, y);
            }
          }
          _ => {}
        })?;
        if should_quit {
          break 'main;
        }
      }

      self
        .renderer
        .borrow_mut()
        .set_draw_color(Color::RGB(253, 243, 226));
      self.renderer.borrow_mut().clear();

      let header_dim = Rect::new(20, 50, client.width() - 40, 100);
      header(header_dim, &self.music_info, self.score.score_point)(
        self.renderer.clone(),
      )?;
      let stats_dim = Rect::new(
        0,
        client.height() as i32 - 300,
        client.width(),
        200,
      );
      stats(0.0, self.score.clone())(
        self.renderer.clone(),
        stats_dim,
      )?;
      {
        const WIDTH: u32 = 240;
        const HEIGHT: u32 = 80;
        const MARGIN: u32 = 20;

        let retry_button_area = Rect::new(
          client.width() as i32 - WIDTH as i32 - MARGIN as i32,
          client.height() as i32 - HEIGHT as i32 - MARGIN as i32,
          WIDTH,
          HEIGHT,
        );
        let on_hover = retry_button_area.contains_point(mouse_pos);

        if on_hover {
          self
            .renderer
            .borrow_mut()
            .set_draw_color(Color::RGB(220, 224, 220));
          self.renderer.borrow_mut().fill_rect(retry_button_area)?;
        }

        if retry_button_area.contains_point(started_pressing)
          && retry_button_area.contains_point(ended_pressing)
        {
          return Ok(ViewRoute::GameView);
        }
        self
          .renderer
          .borrow_mut()
          .set_draw_color(Color::RGB(10, 14, 10));
        self.renderer.borrow_mut().draw_rect(retry_button_area)?;
        use super::renderer::text::TextAlign;
        self.renderer.borrow_mut().text(|style| {
          style
            .align(TextAlign::Center)
            .text("再挑戦")
            .color(Color::RGB(36, 141, 255))
            .line_height(60)
            .pos(client.bottom_right().offset(
              -(WIDTH as i32) / 2 - MARGIN as i32,
              -(HEIGHT as i32) / 2 - MARGIN as i32,
            ))
        })?;
      }

      self.renderer.borrow_mut().flush();

      let draw_time = time.elapsed().as_secs_f64();
      self
        .handler
        .delay((1e3 / 60.0 - draw_time * 1e3).max(0.0) as u32)?;
    }
    Ok(ViewRoute::Quit)
  }
}
