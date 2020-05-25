use super::{
  components::{header, stats},
  handler::Handler,
  renderer::RenderCtx,
  View,
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
  fn run(&mut self) -> Result<(), super::ViewError> {
    let client = Rect::new(
      0,
      0,
      self.renderer.borrow().width(),
      self.renderer.borrow().height(),
    );

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

      let header_dim = Rect::new(0, 0, client.width(), 100);
      header(&self.music_info, self.score.score_point)(
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
        use super::renderer::text::TextAlign;
        self.renderer.borrow_mut().text(|style| {
          style
            .align(TextAlign::Right)
            .text("何かキーを押すと終了")
            .color(Color::RGB(36, 141, 255))
            .line_height(60)
            .pos(client.bottom_right().offset(0, -60))
        })?;
      }

      self.renderer.borrow_mut().flush();

      let draw_time = time.elapsed().as_secs_f64();
      self
        .handler
        .delay((1e3 / 60.0 - draw_time * 1e3).max(0.0) as u32)?;
    }
    Ok(())
  }

  fn next_route(&self) -> Option<super::ViewRoute> {
    Some(super::ViewRoute::Quit)
  }
}
