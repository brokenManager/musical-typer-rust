use crate::{
  model::exp::time::Seconds,
  view::renderer::{text::TextStyle, RenderCtx, ViewResult},
};

pub fn fade_out_text(
  duration: Seconds,
  styler: impl Fn(TextStyle) -> TextStyle,
) -> impl FnMut(RenderCtx, Seconds) -> ViewResult {
  let mut time: Seconds = 0.0.into();
  move |ctx: RenderCtx, delta_time| -> ViewResult {
    time += delta_time;
    let ratio = time.clone() / duration.clone();
    let opacity =
      (u8::max_value() as f64 * (1.0 - ratio).max(0.0)) as u8;

    ctx
      .borrow_mut()
      .text(|style: TextStyle| styler(style).opacity(opacity))?;
    Ok(())
  }
}
