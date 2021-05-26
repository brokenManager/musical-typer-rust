use crate::view::{
  handler::MouseState,
  renderer::{Component, Renderer, ViewResult},
};
use sdl2::{pixels::Color, rect::Rect};

#[derive(PartialEq)]
pub struct ButtonProps {
  pub border_color: Color,
  pub color_on_hover: Color,
  pub mouse: MouseState,
}

pub struct Button<H> {
  props: ButtonProps,
  bounds: Rect,
  on_click: H,
}

impl<H: FnMut()> Button<H> {
  pub fn new(props: ButtonProps, bounds: Rect, on_click: H) -> Self {
    Self {
      props,
      bounds,
      on_click,
    }
  }
}

impl<H: FnMut()> Component for Button<H> {
  type Props = ButtonProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.props != new_props
  }

  fn update(&mut self, props: Self::Props) {
    self.props = props;

    if self
      .bounds
      .contains_point(self.props.mouse.started_pressing)
      && self.bounds.contains_point(self.props.mouse.ended_pressing)
    {
      (self.on_click)();
    }
  }

  fn render(&self, canvas: &mut Renderer<'_, '_>) -> ViewResult {
    let &Button { props, bounds, .. } = &self;
    let &ButtonProps {
      color_on_hover,
      border_color,
      mouse,
    } = &props;

    let on_hover = bounds.contains_point(mouse.mouse_pos);

    if on_hover {
      canvas.set_draw_color(*color_on_hover);
      canvas.fill_rect(*bounds)?;
    }

    canvas.set_draw_color(*border_color);
    canvas.draw_rect(*bounds)?;
    Ok(())
  }
}
