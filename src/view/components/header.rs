use crate::{
  model::exp::scoremap::MusicInfo,
  view::renderer::{text::TextAlign, RenderCtx, ViewResult},
};
use sdl2::rect::Point;

pub fn header<'renderer, 'info: 'renderer>(
  music_info: &'info MusicInfo,
  score_point: i32,
) -> impl Fn(RenderCtx) -> ViewResult + 'renderer {
  move |ctx: RenderCtx| -> ViewResult {
    use sdl2::pixels::Color;

    let title = &music_info.title;
    let author = &music_info.song_author;

    ctx.borrow_mut().text(|s| {
      s.text(title)
        .color(Color::RGB(0, 0, 0))
        .line_height(50)
        .align(TextAlign::Right)
        .pos(Point::new(800, 0))
    })?;

    ctx.borrow_mut().text(|s| {
      s.text(author)
        .color(Color::RGB(156, 156, 162))
        .line_height(50)
        .align(TextAlign::Right)
        .pos(Point::new(800, 50))
    })?;

    ctx.borrow_mut().text(|s| {
      s.text(format!("{:08}", score_point).as_str())
        .color(Color::RGB(64, 79, 181))
        .line_height(50)
        .pos(Point::new(0, 50))
    })?;

    Ok(())
  }
}
