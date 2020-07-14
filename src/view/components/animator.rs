use crate::{
  model::exp::time::Seconds,
  view::renderer::{Component, RenderCtx, ViewResult},
};

mod fade_out_text;

pub use fade_out_text::FadeOutText;

pub trait TransparentComponent: Component {
  fn render(
    &self,
    ctx: RenderCtx<'_, '_>,
    opacity: f64,
  ) -> ViewResult;
}

pub struct FadeOutProps {
  time: Seconds,
}

pub struct FadeOut<ChildProp> {
  props: FadeOutProps,
  child: Box<dyn TransparentComponent<Props = ChildProp>>,
  duration: Seconds,
}

impl<ChildProps> FadeOut<ChildProps> {
  pub fn new(
    child: impl TransparentComponent<Props = ChildProps>,
    duration: Seconds,
  ) -> Self {
    Self {
      props: FadeOutProps { time: 0.0 },
      child: Box::new(child),
      duration,
    }
  }
}

impl Component for FadeOut {
  type Props = FadeOutProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    todo!()
  }

  fn update(&mut self, new_props: Self::Props) {
    self.props = new_props;
  }

  fn render(&self, ctx: RenderCtx<'_, '_>) -> ViewResult {
    let ratio = self.props.time.clone() / self.duration.clone();
    let opacity = (1.0 - ratio).max(0.0);

    self.props.child.render(ctx, opacity);
  }
}
