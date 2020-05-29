use super::{
  components::{button, header, stats},
  handler::Handler,
  renderer::RenderCtx,
  View, ViewRoute,
};
use crate::model::exp::{
  game_activity::GameScore, scoremap::MusicInfo,
};
use sdl2::{pixels::Color, rect::Rect};
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

    enum Dst {
      Game,
      Quit,
    }
    let mut will_navigate_to = None;

    loop {
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
          _ => {}
        })?;
        if should_quit {
          will_navigate_to = Some(Dst::Quit);
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
        button(
          retry_button_area,
          Color::RGB(10, 14, 10),
          Color::RGB(220, 224, 220),
          || {
            will_navigate_to = Some(Dst::Game);
          },
        )(self.renderer.clone(), self.handler.mouse_state())?;

        use super::renderer::text::TextAlign;
        self.renderer.borrow_mut().text(|style| {
          style
            .align(TextAlign::Center)
            .text("再挑戦")
            .color(Color::RGB(36, 141, 255))
            .line_height(60)
            .pos(retry_button_area.center())
        })?;
      }

      self.renderer.borrow_mut().flush();

      let draw_time = time.elapsed().as_secs_f64();
      self
        .handler
        .delay((1e3 / 60.0 - draw_time * 1e3).max(0.0) as u32)?;

      if let Some(will_navigate_to) = will_navigate_to {
        match will_navigate_to {
          Dst::Game => return Ok(ViewRoute::Retry),
          Dst::Quit => return Ok(ViewRoute::Quit),
        }
      }
    }
  }
}
