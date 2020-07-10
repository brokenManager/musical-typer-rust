use crate::{
  model::exp::scoremap::MusicInfo,
  view::renderer::{text::TextAlign, RenderCtx, ViewResult},
};
use sdl2::rect::Rect;

pub fn header<'renderer, 'info: 'renderer>(
  client: Rect,
  music_info: &'info MusicInfo,
  score_point: i32,
) -> impl Fn(RenderCtx) -> ViewResult + 'renderer {
  move |ctx: RenderCtx| -> ViewResult {
    use sdl2::pixels::Color;

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
