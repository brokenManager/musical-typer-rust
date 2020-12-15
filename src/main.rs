#![feature(or_patterns)]

mod model;
mod view;

use model::exp::scoremap::ScoremapError;
use std::{fs::File, path::Path};
use view::ViewError;

#[derive(Debug)]
pub enum EntireError {
  ScoremapError(ScoremapError),
  ViewError(ViewError),
}

impl From<ScoremapError> for EntireError {
  fn from(err: ScoremapError) -> Self {
    EntireError::ScoremapError(err)
  }
}

impl From<ViewError> for EntireError {
  fn from(err: ViewError) -> Self {
    EntireError::ViewError(err)
  }
}

fn main() -> Result<(), EntireError> {
  use model::exp::scoremap::Scoremap;
  let score = Scoremap::from_file(
    File::open(Path::new("score/sampleScore.tsc")).unwrap(),
    |config| config.ignore_unsupported_property(true),
  )?;

  view::run_router(score)?;
  Ok(())
}
