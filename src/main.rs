#![feature(or_patterns)]

mod model;
mod sdl;

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

  use sdl::SDLView;
  let mut presenter = SDLView::new(800, 600).unwrap();
  presenter.run().unwrap();
}
