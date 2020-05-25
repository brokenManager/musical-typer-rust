use super::{
  handler::Handler, renderer::RenderCtx, stats::stats, View,
};
use crate::model::exp::game_activity::GameScore;
use sdl2::{pixels::Color, rect::Rect};
use std::time::Instant;

pub struct ResultView<'ttf, 'canvas> {
  renderer: RenderCtx<'ttf, 'canvas>,
  handler: Handler,
  score: GameScore,
}

impl<'ttf, 'canvas> ResultView<'ttf, 'canvas> {
  pub fn new(
    renderer: RenderCtx<'ttf, 'canvas>,
    handler: Handler,
    score: GameScore,
  ) -> Self {
    Self {
      renderer,
      handler,
      score,
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

      let stats_dim = Rect::new(
        0,
        client.height() as i32 - 200,
        client.width(),
        200,
      );
      stats(0.0, self.score.clone())(
        self.renderer.clone(),
        stats_dim,
      )?;

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
