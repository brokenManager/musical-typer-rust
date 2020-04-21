use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::{
  model::exp::sentence::Sentence,
  view::renderer::{RenderCtx, ViewResult},
};

mod finder;
mod header;
mod keyboard;

use finder::finder;
use header::header;
use keyboard::keyboard;

pub struct WholeProps<'a> {
  pub pressed_keys: &'a [char],
  pub sentence: &'a Option<Sentence>,
  pub title: &'a str,
  pub song_author: &'a str,
  pub score_point: i32,
}

pub fn render<'texture>(
  ctx: RenderCtx<'_, 'texture>,
  client: Rect,
  props: &WholeProps,
) -> ViewResult {
  ctx.borrow_mut().set_draw_color(Color::RGB(253, 243, 226));
  ctx.borrow_mut().clear();

  {
    let header_dim = Rect::new(0, 0, client.width(), 100);
    header(props.title, props.song_author, props.score_point)(
      ctx.clone(),
    )?;
    ctx.borrow_mut().set_draw_color(Color::RGB(0, 0, 0));
    ctx.borrow_mut().draw_rect(header_dim)?;
  }

  {
    let finder_dim = Rect::new(0, 100, client.width(), 200);
    finder(props.sentence, 0.2)(ctx.clone(), finder_dim)?;
    ctx.borrow_mut().set_draw_color(Color::RGB(0, 0, 0));
    ctx.borrow_mut().draw_rect(finder_dim)?;
  }

  {
    let keyboard_dim =
      Rect::new(0, client.height() as i32 - 300, client.width(), 300);
    keyboard(props.pressed_keys, &[])(ctx.clone(), keyboard_dim)?;

    ctx.borrow_mut().set_draw_color(Color::RGB(0, 0, 0));
    ctx.borrow_mut().draw_rect(keyboard_dim)?;
  }

  Ok(())
}
