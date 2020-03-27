mod abst;
mod exp;
mod skin;

use skin::sdl_screen::SDLScreen;

fn main() {
  let mut screen = SDLScreen::new(800, 600);
  screen.run();
}
