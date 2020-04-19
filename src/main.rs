#![feature(or_patterns)]

mod model;
mod view;

use view::{handler::Handler, renderer::Renderer};

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

  use view::Router;
  let sdl = sdl2::init().unwrap();
  let ttf = sdl2::ttf::init().unwrap();
  let mut handler = Handler::new(&sdl);
  let mut renderer = Renderer::new(&sdl, &ttf, 800, 600).unwrap();

  let mut router =
    Router::new(&mut handler, &mut renderer, score).unwrap();
  router.run().unwrap();
}
