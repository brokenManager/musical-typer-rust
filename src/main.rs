#![feature(or_patterns)]

mod exp;
mod op;
mod skin;

fn main() {
  use exp::scoremap::{Scoremap, ScoremapLoadConfig};
  let score = Scoremap::from_file(
    std::fs::File::open(std::path::Path::new(
      "example/sampleScore.tsc",
    ))
    .unwrap(),
    ScoremapLoadConfig::new().ignore_invalid_properties(false),
  )
  .unwrap();

  use skin::sdl::SDLView;
  let mut presenter = SDLView::new(800, 600).unwrap();
  presenter.run().unwrap();
}
