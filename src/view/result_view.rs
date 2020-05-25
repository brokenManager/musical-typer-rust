use super::{handler::Handler, renderer::RenderCtx, View};
use crate::model::exp::game_activity::GameScore;

pub struct ResultView<'ttf, 'canvas> {
  renderer: RenderCtx<'ttf, 'canvas>,
  handler: Handler,
  score: GameScore,
}

impl<'ttf, 'canvas> ResultView<'ttf, 'canvas> {
  pub fn new(
    renderer: RenderCtx<'ttf, 'canvas>,
    handler: Handler,
    score: GameScore,
  ) -> Self {
    Self {
      renderer,
      handler,
      score,
    }
  }
}

impl<'ttf, 'canvas> View for ResultView<'ttf, 'canvas> {
  fn run(&mut self) -> Result<(), super::ViewError> {
    
    Ok(())
  }

  fn next_route(&self) -> Option<super::ViewRoute> {
    Some(super::ViewRoute::Quit)
  }
}
