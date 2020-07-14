use super::TransparentComponent;
use crate::view::renderer::{
  text::TextStyle, Component, RenderCtx, ViewResult,
};

pub struct FadeOutText {
  styler: Box<dyn Fn(TextStyle) -> TextStyle>,
}

impl Component for FadeOutText {
  type Props = Self;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    true
  }

  fn update(&mut self, new_props: Self::Props) {}

  fn render(&self, ctx: RenderCtx<'_, '_>) -> ViewResult {
    ctx.borrow_mut().text(|style| self.styler(style))?;
  }
}

impl TransparentComponent for FadeOutText {
  fn render(
    &self,
    ctx: RenderCtx<'_, '_>,
    opacity: f64,
  ) -> ViewResult {
    let opacity = (u8::max_value() as f64 * opacity) as u8;

    ctx
      .borrow_mut()
      .text(|style| self.styler(style).opacity(opacity))?;
  }
}
