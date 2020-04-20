#![feature(or_patterns)]

mod model;
mod view;

fn main() {
  use model::exp::scoremap::Scoremap;
  let score = Scoremap::from_file(
    std::fs::File::open(std::path::Path::new(
      "score/sampleScore.tsc",
    ))
    .unwrap(),
    |config| config.ignore_invalid_properties(true),
  )
  .unwrap();

  view::run_router(score).unwrap();
}
