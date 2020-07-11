use crate::{
  model::exp::scoremap::MusicInfo,
  view::renderer::{
    text::TextAlign, Component, RenderCtx, ViewResult,
  },
};
use sdl2::{pixels::Color, rect::Rect};

#[derive(PartialEq)]
pub struct HeaderProps {
  pub music_info: MusicInfo,
  pub score_point: i32,
}

pub struct Header {
  props: HeaderProps,
  client: Rect,
}

impl Header {
  pub fn new(props: HeaderProps, client: Rect) -> Self {
    Self { props, client }
  }
}

impl Component for Header {
  type Props = HeaderProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.props == new_props
  }

  fn update(&mut self, new_props: Self::Props) {
    self.props = new_props;
  }

  fn render(&self, ctx: RenderCtx<'_, '_>) -> ViewResult {
    let &Header { props, client } = &self;
    let &HeaderProps {
      music_info,
      score_point,
    } = &props;

    let mut canvas = ctx.borrow_mut();

    let title = &music_info.title;
    let author = &music_info.song_author;

    canvas.text(|s| {
      s.text(title)
        .color(Color::RGB(0, 0, 0))
        .line_height(60)
        .align(TextAlign::Right)
        .pos(client.top_right().offset(-5, 5))
    })?;

    canvas.text(|s| {
      s.text(author)
        .color(Color::RGB(156, 156, 162))
        .line_height(30)
        .align(TextAlign::Right)
        .pos(client.bottom_right().offset(-5, -35))
    })?;

    canvas.text(|s| {
      s.text(format!("{:08}", score_point).as_str())
        .color(Color::RGB(64, 79, 181))
        .line_height(70)
        .pos(client.bottom_left().offset(5, -60))
    })?;

    Ok(())
  }
}
