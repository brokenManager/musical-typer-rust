use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::{
  model::exp::{
    game_activity::GameScore, scoremap::MusicInfo, sentence::Sentence,
  },
  view::{
    components::{header, stats},
    renderer::{RenderCtx, ViewResult},
  },
};

mod finder;
mod keyboard;

use finder::finder;
use keyboard::keyboard;

pub struct WholeProps<'a> {
  pub pressed_keys: &'a [char],
  pub sentence: &'a Sentence,
  pub music_info: MusicInfo,
  pub type_per_second: f64,
  pub score: GameScore,
  pub section_remaining_ratio: f64,
}

pub fn render<'texture>(
  ctx: RenderCtx<'_, 'texture>,
  props: &WholeProps,
) -> ViewResult {
  let mut canvas = ctx.borrow_mut();

  let client =
    sdl2::rect::Rect::new(0, 0, canvas.width(), canvas.height());

  canvas.set_draw_color(Color::RGB(253, 243, 226));
  canvas.clear();

  {
    let header_dim = Rect::new(0, 0, client.width(), 100);
    header(header_dim, &props.music_info, props.score.score_point)(
      ctx.clone(),
    )?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_rect(header_dim)?;
  }

  {
    let finder_dim = Rect::new(0, 100, client.width(), 150);
    finder(props.sentence, props.section_remaining_ratio)(
      ctx.clone(),
      finder_dim,
    )?;
  }

  {
    let hint = props
      .sentence
      .roman()
      .will_input
      .chars()
      .next()
      .map_or(vec![], |c| vec![c]);
    let keyboard_dim =
      Rect::new(0, client.height() as i32 - 350, client.width(), 200);
    keyboard(props.pressed_keys, hint.as_slice())(
      ctx.clone(),
      keyboard_dim,
    )?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_rect(keyboard_dim)?;
  }
  {
    let stats_dim =
      Rect::new(0, client.height() as i32 - 150, client.width(), 150);
    stats(props.type_per_second, props.score.clone())(
      ctx.clone(),
      stats_dim,
    )?;
  }

  Ok(())
}
