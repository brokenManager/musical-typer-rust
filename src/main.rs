#![feature(or_patterns)]

mod controller;
mod model;
mod sdl;

fn main() {
  use controller::MTController;
  let mut controller = MTController::new();

  use model::exp::scoremap::Scoremap;
  let score = Scoremap::from_file(
    std::fs::File::open(std::path::Path::new(
      "example/sampleScore.tsc",
    ))
    .unwrap(),
    |config| config.ignore_invalid_properties(true),
  )
  .unwrap();

  controller.run(score).unwrap();
}
