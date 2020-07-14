use super::TextStyle;
use crate::{
  model::exp::time::Seconds,
  view::renderer::{Component, RenderCtx, ViewResult},
};

pub trait TextAnimator {
  fn style(style: TextStyle, progress: f64) -> TextStyle;
}

pub enum AnimatorCombinator<T> {
  One(T),
  More(T, Box<AnimatorCombinator<T>>),
}

impl<T> AnimatorCombinator<T> {
  pub fn connect(self, animator: T) -> Self {
    AnimatorCombinator::More(animator, Box::new(self))
  }

  pub fn drain<R, F>(&self, init: R, f: F) -> R
  where
    F: Fn(R, &T) -> R,
  {
    match self {
      AnimatorCombinator::One(one) => f(init, one),
      AnimatorCombinator::More(one, next) => {
        let after_one = f(init, one);
        next.drain(after_one, f)
      }
    }
  }
}

impl<T> AnimatorCombinator<T>
where
  T: TextAnimator,
{
  pub fn style(&self, progress: f64) -> TextStyle {
    let init = TextStyle::new();
    self.drain(init, |style, styler| styler.style(progress, style));
  }
}

type TextAnimatorCombinator = AnimatorCombinator<dyn TextAnimator>;

#[derive(PartialEq)]
pub struct AnimatedTextProps {
  curr_time: Seconds,
  duration: Seconds,
}

pub struct AnimatedText {
  props: AnimatedTextProps,
  animator: TextAnimatorCombinator,
}

impl AnimatedText {}

impl Component for AnimatedText {
  type Props = AnimatedTextProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self != new_props
  }

  fn update(&mut self, new_props: Self::Props) {
    self = new_props;
  }

  fn render(&self, ctx: RenderCtx<'_, '_>) -> ViewResult {
    todo!()
  }
}
