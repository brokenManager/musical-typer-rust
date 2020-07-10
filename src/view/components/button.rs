use crate::view::{
  handler::MouseState,
  renderer::{RenderCtx, ViewResult},
};
use sdl2::{pixels::Color, rect::Rect};

pub fn button<'renderer, H: 'renderer>(
  bounds: Rect,
  border_color: Color,
  color_on_hover: Color,
  mut on_click: H,
) -> impl FnMut(RenderCtx, &MouseState) -> ViewResult + 'renderer
where
  H: FnMut(),
{
  move |ctx, mouse| {
    let mut canvas = ctx.borrow_mut();
    let on_hover = bounds.contains_point(mouse.mouse_pos);

    if on_hover {
      canvas.set_draw_color(color_on_hover);
      canvas.fill_rect(bounds)?;
    }

    if bounds.contains_point(mouse.started_pressing)
      && bounds.contains_point(mouse.ended_pressing)
    {
      on_click();
    }
    canvas.set_draw_color(border_color);
    canvas.draw_rect(bounds)?;
    Ok(())
  }
}
