#[cfg(test)]
mod TestEnv {
  use crate::abst::controller::Controller;
  use crate::abst::presenter::Presenter;

  struct KeyPress(f64, &'static str);

  struct MockController {
    key_press_schedule: &'static [KeyPress],
  }

  impl MockController {
    fn new(key_press_schedule: &[KeyPress]) -> Self {
      MockController { key_press_schedule }
    }
  }

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

    let mut controller = MockController::new(&[
      KeyPress(3.0, "moudamedasonnnatokiha"),
      KeyPress(6.5, "anosorawomiagetegorann"),
      KeyPress(11.0, "yorunoyamiwoosiagete"),
      KeyPress(14.5, "taiyougamatahohoemikureru"),
      KeyPress(19.0, "maedakemitetemortukarerune"),
      KeyPress(22.5, "tamanihatatidomatteiinndayo"),
      KeyPress(27.0, "muneippaikuukisuttara"),
      KeyPress(30.75, "mataashiwohumidasouyo"),
      KeyPress(35.0, "bokuranoyumesorawokoete"),
      KeyPress(39.0, "hateshinakuhirogatteikuyo"),
      KeyPress(43.0, "namidanoatomomunenoitamimo"),
      KeyPress(47.0, "kiminochikaraninaru"),
      KeyPress(51.0, "maltukuradanagedasumaeni"),
      KeyPress(54.0, "anosorawomiagetegorann"),
      KeyPress(59.0, "yorunoyamimewokoraseba"),
      KeyPress(62.5, "hoshitachinodannsupa-texi-"),
      KeyPress(67.0, "tuyogaribakarijatukarerune"),
      KeyPress(71.0, "namidawokoboshitemoiinndayo"),
      KeyPress(75.0, "omoikirinaitaatoniha"),
      KeyPress(78.75, "mataegaowomisetene"),
      KeyPress(83.0, "minnnanoyumetokiwokoete"),
      KeyPress(87.0, "dokomademotunagaxtuteikuyo"),
      KeyPress(91.0, "namidanoatomomunenoitamimo"),
      KeyPress(95.0, "kiminochikaraninaru"),
      KeyPress(99.0, "sukoshidutumaenisusumou"),
      KeyPress(103.0, "miraihazuttomatteirukara"),
      KeyPress(107.0, "omoikirinaitayorusase"),
      KeyPress(110.5, "itukaomoidenikawaruyo"),
      KeyPress(117.0, "bokuranoyumesorawokoete"),
      KeyPress(121.0, "hateshinakuhirogatteikuyo"),
      KeyPress(125.0, "namidanoatomomunenoitamimo"),
      KeyPress(129.0, "kiminochikaraninaru"),
      KeyPress(133.0, "minnnanoyumetokiwokoete"),
      KeyPress(137.0, "dokomademotunagatteikuyo"),
      KeyPress(141.0, "namidanoatomomunenoitamimo"),
      KeyPress(145.0, "kiminochikaraninaru"),
      KeyPress(149.0, "namidanoatomomunenoitamimo"),
      KeyPress(153.0, "kiminochikaraninaru"),
    ]);
    let mut presenter = MockPresenter::new();

    game.run_game(&mut controller, &mut presenter);
  }
}
