#[cfg(test)]
mod TestEnv {
  use crate::abst::controller::Controller;
  use crate::abst::presenter::Presenter;

  struct MockController;

  impl Controller for MockController {
    fn key_press(&mut self) -> char {
      unimplemented!()
    }
    fn elapse_time(&mut self) -> f64 {
      unimplemented!()
    }
  }

  struct MockPresenter;

  impl Presenter for MockPresenter {
    fn play_bgm(&mut self, name: &str) {
      unimplemented!()
    }
    fn decrease_remaining_time(&mut self, delta_time: f64) {
      unimplemented!()
    }
    fn update_string_to_input(
      &mut self,
      string: &crate::exp::string_to_input::StringToInput,
    ) {
      unimplemented!()
    }
    fn mistyped(&mut self) {
      unimplemented!()
    }
    fn flush_screen(&mut self) {
      unimplemented!()
    }
  }

  #[test]
  fn op1() {
    use crate::exp::scoremap::Scoremap;
    use crate::op::on_game::MusicalTyper;

    let test_score = Scoremap::from_file(
      std::fs::File::open(std::path::Path::new(
        "examples/sampleScore.tsc",
      ))
      .unwrap(),
    )
    .unwrap();

    let game = MusicalTyper::new(test_score);

    let mut controller = MockController::new();
    let mut presenter = MockPresenter::new();

    game.run_game(&mut controller, &mut presenter);
  }
}
