mod abst;
mod exp;
mod op;
mod skin;

fn main() {
  use skin::sdl_presenter::SDLPresenter;
  let mut presenter = SDLPresenter::new(800, 600).unwrap();
  presenter.run().unwrap();
}
