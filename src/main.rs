mod abst;
mod exp;
mod op;
mod skin;

use skin::sdl_presenter::SDLPresenter;

fn main() {
  let mut screen = SDLPresenter::new(800, 600).unwrap();
  screen.run().unwrap();
}
