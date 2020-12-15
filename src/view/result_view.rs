use super::{
  components::{
    Button, ButtonProps, Header, HeaderProps, Stats, StatsProps,
  },
  handler::Handler,
  renderer::{Component, RenderCtx},
  View, ViewRoute,
};
use crate::model::exp::{
  game_activity::GameScore, scoremap::MusicInfo,
};
use sdl2::{pixels::Color, rect::Rect};
use std::{cell::RefCell, rc::Rc, time::Instant};

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
    let will_navigate_to = Rc::new(RefCell::new(None));

    let stats_dim =
      Rect::new(0, client.height() as i32 - 300, client.width(), 200);
    let mut stats = Stats::new(
      StatsProps {
        type_per_second: 0.0,
        score: self.score.clone(),
      },
      stats_dim,
    );

    let header_dim = Rect::new(20, 50, client.width() - 40, 100);
    let mut header = Header::new(
      HeaderProps {
        music_info: self.music_info.clone(),
        score_point: self.score.score_point,
      },
      header_dim,
    );

    const WIDTH: u32 = 240;
    const HEIGHT: u32 = 80;
    const MARGIN: u32 = 20;
    let retry_button_area = Rect::new(
      client.width() as i32 - WIDTH as i32 - MARGIN as i32,
      client.height() as i32 - HEIGHT as i32 - MARGIN as i32,
      WIDTH,
      HEIGHT,
    );
    let mut retry_button = Button::new(
      ButtonProps {
        border_color: Color::RGB(10, 14, 10),
        color_on_hover: Color::RGB(220, 224, 220),
        mouse: self.handler.mouse_state().clone(),
      },
      retry_button_area,
      || {
        will_navigate_to.borrow_mut().replace(Dst::Game);
      },
    );

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
          will_navigate_to.borrow_mut().replace(Dst::Quit);
        }
      }

      self
        .renderer
        .borrow_mut()
        .set_draw_color(Color::RGB(253, 243, 226));
      self.renderer.borrow_mut().clear();

      header.update(HeaderProps {
        music_info: self.music_info.clone(),
        score_point: self.score.score_point,
      });
      header.render(&mut self.renderer.borrow_mut())?;

      stats.update(StatsProps {
        type_per_second: 0.0,
        score: self.score.clone(),
      });
      stats.render(&mut self.renderer.borrow_mut())?;

      {
        let new_props = ButtonProps {
          border_color: Color::RGB(10, 14, 10),
          color_on_hover: Color::RGB(220, 224, 220),
          mouse: self.handler.mouse_state().clone(),
        };
        if retry_button.is_needed_redraw(&new_props) {
          retry_button.update(new_props);
        }
        retry_button.render(&mut self.renderer.borrow_mut())?;

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

      let will_navigate_to = will_navigate_to.borrow();
      if let Some(will_navigate_to) = will_navigate_to.as_ref() {
        match will_navigate_to {
          Dst::Game => return Ok(ViewRoute::Retry),
          Dst::Quit => return Ok(ViewRoute::Quit),
        }
      }
    }
  }
}
