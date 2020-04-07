use super::exp::game_activity::GameActivity;
use super::exp::minute_second::Seconds;
use super::exp::roman::roman_lexer::RomanParseError;
use super::exp::scoremap::lexer::ScoremapLexError;
use super::exp::scoremap::{Scoremap, ScoremapError};
use super::exp::sentence::Sentence;

pub trait Controller {
  fn key_press(&mut self) -> Vec<char>;
  fn elapse_time(&mut self) -> f64;
}
pub trait Presenter {
  fn play_bgm(&mut self, name: &str);
  fn decrease_remaining_time(&mut self, delta_time: f64);
  fn update_sentence(&mut self, string: &Sentence);
  fn pointed(&mut self, added: i32);
  fn typed(&mut self, is_mistaken: bool);
  fn flush_screen(&mut self);
}

#[derive(Debug)]
pub enum MusicalTyperError {
  SongDataNotFound,
  FileReadError { reason: String },
  ScoremapBuildError(ScoremapError),
}

use MusicalTyperError::*;

impl From<std::io::Error> for MusicalTyperError {
  fn from(err: std::io::Error) -> Self {
    MusicalTyperError::FileReadError {
      reason: err.to_string(),
    }
  }
}

impl From<ScoremapError> for MusicalTyperError {
  fn from(err: ScoremapError) -> Self {
    MusicalTyperError::ScoremapBuildError(err)
  }
}
impl From<RomanParseError> for MusicalTyperError {
  fn from(_err: RomanParseError) -> Self {
    MusicalTyperError::ScoremapBuildError(ScoremapError::LexError(
      ScoremapLexError::InvalidStatementDefinition {
        line_num: 1,
        reason: "ふりがなでのそのような平仮名の並びは未対応です。",
      },
    ))
  }
}

pub type Point = u32;
pub type TypingSpeed = f64;

pub struct MusicalTyperConfig {
  wrong_type: Point,
  correct_type: Point,
  missed_sentence: Point,
  complete_sentence: Point,
  perfect_sentence: Point,
  perfect_section: Point,
  ideal_type: TypingSpeed,
}

impl Default for MusicalTyperConfig {
  fn default() -> Self {
    MusicalTyperConfig {
      wrong_type: 30,
      correct_type: 10,
      missed_sentence: 2,
      complete_sentence: 50,
      perfect_sentence: 100,
      perfect_section: 300,
      ideal_type: 3.0,
    }
  }
}

pub struct MusicalTyper<'p, P> {
  score: Scoremap,
  activity: GameActivity,
  accumulated_time: Seconds,
  presenter: &'p mut P,
  config: MusicalTyperConfig,
}

impl<'p, P> MusicalTyper<'p, P>
where
  P: Presenter,
{
  pub fn new(
    score: Scoremap,
    presenter: &'p mut P,
    config: MusicalTyperConfig,
  ) -> Result<Self, MusicalTyperError> {
    let activity = GameActivity::new(&score.notes);

    let metadata = &score.metadata;
    if let Some(ref song_data) = metadata.get("song_data") {
      presenter.play_bgm(song_data);
    } else {
      return Err(SongDataNotFound);
    }

    Ok(MusicalTyper {
      score,
      activity,
      accumulated_time: 0.0,
      presenter,
      config,
    })
  }

  pub fn update(&mut self) {
    if let Some(_) = self.activity.current_section() {
      if let Some(sentence) = self.activity.current_sentence() {
        self.presenter.update_sentence(sentence);
      }
    }
  }

  pub fn key_press(&mut self, typed: Vec<char>) {
    for typed in typed.iter() {
      use super::exp::note::TypeResult::*;
      match self.activity.input(*typed) {
        Succeed => {
          self.presenter.typed(false);
        }
        Mistaken => {
          self.presenter.typed(true);
        }
        Vacant => {}
      }
    }
  }
  pub fn elapse_time(&mut self, delta_time: f64) {
    self.accumulated_time += delta_time;
    self.activity.update_time(self.accumulated_time);
    self.presenter.decrease_remaining_time(delta_time);
  }
}

#[cfg(test)]
mod tests {
  use super::super::exp::sentence::Sentence;
  use super::{MusicalTyperError, Presenter};

  struct KeyPress(f64, &'static str);

  #[derive(Debug, PartialEq)]
  enum PresentLog {
    PlayBGM(String),
    DecreaseRemainingTime(f64),
    UpdateSentence(Sentence),
    Pointed(i32),
    Typed(bool),
  }

  use PresentLog::*;

  struct MockPresenter {
    expected: Vec<PresentLog>,
    index: usize,
  }

  impl MockPresenter {
    fn new(expected: Vec<PresentLog>) -> Self {
      MockPresenter { expected, index: 0 }
    }

    fn log(&mut self, log: PresentLog) {
      assert_eq!(
        self.expected[self.index], log,
        "index: {}",
        self.index
      );
      println!("{:?}; index: {}", log, self.index);
      self.index += 1;
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
    fn pointed(&mut self, added: i32) {
      self.log(Pointed(added));
    }
    fn typed(&mut self, is_mistaken: bool) {
      self.log(Typed(is_mistaken));
    }
    fn flush_screen(&mut self) {}
  }

  #[test]
  fn op1() -> Result<(), MusicalTyperError> {
    use super::super::exp::scoremap::Scoremap;
    use super::{MusicalTyper, MusicalTyperConfig};

    let test_score = Scoremap::from_str(
      r#"
# Sample 1
:title TEST
:score_author Mikuro さいな
:song_data void.ogg
:bpm 222.22

[start]
*2.22
打鍵テスト
:だけんてすと
"#,
      |config| config.ignore_invalid_properties(true),
    )?;

    let keypresses = &[KeyPress(2.22, "dakentesuto")];
    let mut presenter = MockPresenter::new(vec![
      PlayBGM("void.ogg".to_owned()),
      DecreaseRemainingTime(2.22),
      Typed(false),
      Typed(false),
      Typed(false),
      Typed(false),
      Typed(false),
      Typed(false),
      Typed(false),
      Typed(false),
      Typed(false),
      Typed(false),
      Typed(false),
      UpdateSentence(Sentence::new("打鍵テスト", "だけんてすと")?),
    ]);

    let mut game = MusicalTyper::new(
      test_score,
      &mut presenter,
      MusicalTyperConfig::default(),
    )?;

    for KeyPress(time, key) in keypresses.iter() {
      game.elapse_time(*time);
      game.key_press(key.chars().collect());
      game.update();
    }

    Ok(())
  }
}
