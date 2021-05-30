use sdl2::{pixels::Color, rect::Rect};

use crate::{
  model::exp::{
    game_activity::GameScore, scoremap::MusicInfo, sentence::Sentence,
  },
  view::{
    components::{Header, HeaderProps, Stats, StatsProps},
    renderer::{Component, Renderer, ViewResult},
  },
};

mod finder;
mod keyboard;

use finder::{Finder, FinderProps};
use keyboard::{Keyboard, KeyboardProps};

#[derive(PartialEq)]
pub struct WholeProps {
  pub pressed_keys: Vec<char>,
  pub sentence: Sentence,
  pub music_info: MusicInfo,
  pub type_per_second: f64,
  pub score: GameScore,
  pub section_remaining_ratio: f64,
}

pub struct Whole {
  keyboard: Keyboard,
  finder: Finder,
  header: Header,
  stats: Stats,
  client: Rect,
}

impl Whole {
  pub fn new(props: WholeProps, client: Rect) -> Self {
    let hint = {
      let roman = props.sentence.roman();
      roman.will_input.chars().next().map_or(vec![], |c| vec![c])
    };
    let keyboard_dim =
      Rect::new(0, client.height() as i32 - 350, client.width(), 200);

    let keyboard = Keyboard::new(
      KeyboardProps {
        pressed_keys: props.pressed_keys.clone(),
        highlighted_keys: hint,
      },
      keyboard_dim,
    );

    let finder_dim = Rect::new(0, 100, client.width(), 150);
    let finder = Finder::new(
      FinderProps {
        sentence: props.sentence.clone(),
        remaining_ratio: props.section_remaining_ratio,
      },
      finder_dim,
    );

    let header_dim = Rect::new(0, 0, client.width(), 100);
    let header = Header::new(
      HeaderProps {
        music_info: props.music_info.clone(),
        score_point: props.score.score_point,
      },
      header_dim,
    );

    let stats_dim =
      Rect::new(0, client.height() as i32 - 150, client.width(), 150);
    let stats = Stats::new(
      StatsProps {
        type_per_second: props.type_per_second,
        score: props.score,
      },
      stats_dim,
    );

    Self {
      keyboard,
      finder,
      header,
      stats,
      client,
    }
  }
}

impl Component for Whole {
  type Props = WholeProps;

  fn is_needed_redraw(&self, _: &Self::Props) -> bool {
    true
  }

  fn update(&mut self, props: Self::Props) {
    let hint = {
      let roman = props.sentence.roman();
      roman.will_input.chars().next().map_or(vec![], |c| vec![c])
    };

    self.keyboard.update(KeyboardProps {
      pressed_keys: props.pressed_keys.clone(),
      highlighted_keys: hint,
    });

    self.finder.update(FinderProps {
      sentence: props.sentence.clone(),
      remaining_ratio: props.section_remaining_ratio,
    });

    self.stats.update(StatsProps {
      type_per_second: props.type_per_second,
      score: props.score,
    });
  }

  fn render(&self, ctx: &mut Renderer<'_, '_>) -> ViewResult {
    let &Whole { client, .. } = &self;

    ctx.set_draw_color(Color::RGB(253, 243, 226));
    ctx.clear();

    {
      let header_dim = Rect::new(0, 0, client.width(), 100);
      self.header.render(ctx)?;
      ctx.set_draw_color(Color::RGB(0, 0, 0));
      ctx.draw_rect(header_dim)?;
    }

    self.finder.render(ctx)?;

    {
      let keyboard_dim = Rect::new(
        0,
        client.height() as i32 - 350,
        client.width(),
        200,
      );
      self.keyboard.render(ctx)?;

      ctx.set_draw_color(Color::RGB(0, 0, 0));
      ctx.draw_rect(keyboard_dim)?;
    }

    self.stats.render(ctx)?;

    Ok(())
  }
}
