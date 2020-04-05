use crate::exp::game_activity::GameActivity;
use crate::exp::minute_second::Seconds;
use crate::exp::note::Section;
use crate::exp::scoremap::Scoremap;
use crate::exp::sentence::Sentence;

pub trait Controller {
  fn key_press(&mut self) -> Vec<char>;
  fn elapse_time(&mut self) -> f64;
}
pub trait Presenter {
  fn play_bgm(&mut self, name: &str);
  fn decrease_remaining_time(&mut self, delta_time: f64);
  fn update_sentence(&mut self, string: &Sentence);
  fn mistyped(&mut self);
  fn flush_screen(&mut self);
}

pub struct MusicalTyper {
  score: Scoremap,
  activity: GameActivity,
  accumulated_time: Seconds,
}

impl MusicalTyper {
  pub fn new(score: Scoremap) -> Self {
    let activity = GameActivity::new(&score.notes);
    MusicalTyper {
      score,
      activity,
      accumulated_time: 0.0,
    }
  }

  pub fn run_game(
    &mut self,
    controller: &mut impl Controller,
    presenter: &mut impl Presenter,
  ) -> Result<(), String> {
    let metadata = &self.score.metadata;
    if let Some(ref bgm) = metadata.get("song_data") {
      presenter.play_bgm(bgm);
    } else {
      return Err("no BGM is found".to_owned());
    }

    self.activity.update_time(0.0);
    while let Some(Section {
      foreign_note,
      from,
      to,
    }) = self.activity.current_section()
    {
      let delta_time = controller.elapse_time();
      self.accumulated_time += delta_time;
      self.activity.update_time(self.accumulated_time);

      for typed in controller.key_press().iter() {
        self.activity.input(*typed);
      }

      presenter.decrease_remaining_time(delta_time);
      if let Some(sentence) = self.activity.current_sentence() {
        presenter.update_sentence(sentence);
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::Controller;
  use super::Presenter;
  use crate::exp::sentence::Sentence;

  struct KeyPress(f64, &'static str);

  struct MockController {
    key_press_schedule: &'static [KeyPress],
  }

  impl MockController {
    fn new(key_press_schedule: &'static [KeyPress]) -> Self {
      MockController { key_press_schedule }
    }
  }

  impl Controller for MockController {
    fn key_press(&mut self) -> Vec<char> {
      let res = self.key_press_schedule[0].1.chars().collect();
      self.key_press_schedule = &self.key_press_schedule[1..];
      println!("{}", self.key_press_schedule.len());
      res
    }
    fn elapse_time(&mut self) -> f64 {
      self.key_press_schedule[0].0
    }
  }

  #[derive(Debug, PartialEq)]
  enum PresentLog {
    PlayBGM(String),
    DecreaseRemainingTime(f64),
    UpdateSentence(Sentence),
    Mistyped,
  }

  use PresentLog::*;

  struct MockPresenter {
    log: Vec<PresentLog>,
  }

  impl MockPresenter {
    fn new() -> Self {
      MockPresenter { log: vec![] }
    }

    fn log(&mut self, log: PresentLog) {
      println!("{:#?}", log);
      self.log.push(log);
    }

    fn logs(&self) -> &[PresentLog] {
      &self.log
    }
  }

  impl Presenter for MockPresenter {
    fn play_bgm(&mut self, name: &str) {
      self.log(PlayBGM(name.to_owned()));
    }
    fn decrease_remaining_time(&mut self, delta_time: f64) {
      self.log(DecreaseRemainingTime(delta_time));
    }
    fn update_sentence(&mut self, string: &Sentence) {
      self.log(UpdateSentence(string.clone()));
    }
    fn mistyped(&mut self) {
      self.log(Mistyped)
    }
    fn flush_screen(&mut self) {}
  }

  #[test]
  fn op1() {
    use crate::exp::scoremap::Scoremap;
    use crate::op::on_game::MusicalTyper;

    let test_score = Scoremap::from_file(
      std::fs::File::open(std::path::Path::new(
        "example/sampleScore.tsc",
      ))
      .unwrap(),
      |config| config.ignore_invalid_properties(true),
    )
    .unwrap();

    let mut game = MusicalTyper::new(test_score);

    let mut controller = MockController::new(&[
      KeyPress(3.0, "moudamedasonnnatokiha"),
      KeyPress(3.5, "anosorawomiagetegorann"),
      KeyPress(4.5, "yorunoyamiwoosiagete"),
      KeyPress(3.5, "taiyougamatahohoemikureru"),
      KeyPress(4.5, "maedakemitetemortukarerune"),
      KeyPress(3.5, "tamanihatatidomatteiinndayo"),
      KeyPress(4.5, "muneippaikuukisuttara"),
      KeyPress(3.75, "mataashiwohumidasouyo"),
      KeyPress(4.25, "bokuranoyumesorawokoete"),
      KeyPress(4.0, "hateshinakuhirogatteikuyo"),
      KeyPress(4.0, "namidanoatomomunenoitamimo"),
      KeyPress(4.0, "kiminochikaraninaru"),
      KeyPress(4.0, "maltukuradanagedasumaeni"),
      KeyPress(3.0, "anosorawomiagetegorann"),
      KeyPress(5.0, "yorunoyamimewokoraseba"),
      KeyPress(3.5, "hoshitachinodannsupa-texi-"),
      KeyPress(3.5, "tuyogaribakarijatukarerune"),
      KeyPress(4.0, "namidawokoboshitemoiinndayo"),
      KeyPress(4.0, "omoikirinaitaatoniha"),
      KeyPress(3.75, "mataegaowomisetene"),
      KeyPress(4.25, "minnnanoyumetokiwokoete"),
      KeyPress(4.0, "dokomademotunagaxtuteikuyo"),
      KeyPress(4.0, "namidanoatomomunenoitamimo"),
      KeyPress(4.0, "kiminochikaraninaru"),
      KeyPress(4.0, "sukoshidutumaenisusumou"),
      KeyPress(4.0, "miraihazuttomatteirukara"),
      KeyPress(4.0, "omoikirinaitayorusase"),
      KeyPress(3.5, "itukaomoidenikawaruyo"),
      KeyPress(6.5, "bokuranoyumesorawokoete"),
      KeyPress(4.0, "hateshinakuhirogatteikuyo"),
      KeyPress(4.0, "namidanoatomomunenoitamimo"),
      KeyPress(4.0, "kiminochikaraninaru"),
      KeyPress(4.0, "minnnanoyumetokiwokoete"),
      KeyPress(4.0, "dokomademotunagatteikuyo"),
      KeyPress(4.0, "namidanoatomomunenoitamimo"),
      KeyPress(4.0, "kiminochikaraninaru"),
      KeyPress(4.0, "namidanoatomomunenoitamimo"),
      KeyPress(4.0, "kiminochikaraninaru"),
    ]);
    let mut presenter = MockPresenter::new();

    game.run_game(&mut controller, &mut presenter).unwrap();

    assert_eq!(
      presenter.logs(),
      &[
        PlayBGM("kkiminochikara-edited.wav".to_owned()),
        DecreaseRemainingTime(3.0),
        UpdateSentence(
          Sentence::new(
            "もうダメだ そんな時は",
            "もうだめだそんなときは"
          )
          .unwrap()
        )
      ]
    );
  }
}
