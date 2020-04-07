#![feature(or_patterns)]

mod model;
mod view;

fn main() {
  use model::exp::scoremap::Scoremap;
  let score = Scoremap::from_file(
    std::fs::File::open(std::path::Path::new(
      "example/sampleScore.tsc",
    ))
    .unwrap(),
    |config| config.ignore_invalid_properties(true),
  )
  .unwrap();

  let mut view = view::GameView::new(800, 600, score).unwrap();
  view.run().unwrap();
}
