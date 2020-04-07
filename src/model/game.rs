use super::exp::game_activity::GameActivity;
use super::exp::minute_second::Seconds;
use super::exp::roman::roman_lexer::RomanParseError;
use super::exp::scoremap::lexer::ScoremapLexError;
use super::exp::scoremap::{Scoremap, ScoremapError};
use super::exp::sentence::Sentence;

#[derive(Debug, Clone, PartialEq)]
pub enum MusicalTyperEvent {
  PlayBgm(String),
  UpdateSentence(Sentence),
  Pointed(i32),
  Typed { mistaken: bool },
}

use MusicalTyperEvent::*;

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

pub struct MusicalTyper {
  activity: GameActivity,
  accumulated_time: Seconds,
  event_queue: Vec<MusicalTyperEvent>,
  config: MusicalTyperConfig,
}

impl MusicalTyper {
  pub fn new(
    score: &Scoremap,
    config: MusicalTyperConfig,
  ) -> Result<Self, MusicalTyperError> {
    let mut event_queue = vec![];
    let activity = GameActivity::new(&score.notes);

    let metadata = &score.metadata;
    if let Some(song_data) = metadata.get("song_data") {
      event_queue.push(PlayBgm(song_data.to_owned()));
    } else {
      return Err(SongDataNotFound);
    }

    Ok(MusicalTyper {
      activity,
      accumulated_time: 0.0,
      event_queue,
      config,
    })
  }

  #[must_use]
  pub fn key_press(
    &mut self,
    typed: impl Iterator<Item = char>,
  ) -> Vec<MusicalTyperEvent> {
    for typed in typed {
      use super::exp::note::TypeResult::*;
      match self.activity.input(typed) {
        Succeed => {
          self.event_queue.append(&mut vec![
            Pointed(self.config.correct_type as i32),
            Typed { mistaken: false },
          ]);
        }
        Mistaken => {
          self.event_queue.append(&mut vec![
            Pointed(-(self.config.wrong_type as i32)),
            Typed { mistaken: true },
          ]);
        }
        Vacant => {}
      }
    }
    self.pack_events()
  }

  #[must_use]
  pub fn elapse_time(
    &mut self,
    delta_time: f64,
  ) -> Vec<MusicalTyperEvent> {
    self.accumulated_time += delta_time;
    self.activity.update_time(self.accumulated_time);
    self.pack_events()
  }

  fn pack_events(&mut self) -> Vec<MusicalTyperEvent> {
    if let Some(sentence) = self.activity.current_sentence() {
      self.event_queue.push(UpdateSentence(sentence.clone()));
    }

    let res = self.event_queue.iter().cloned().collect();
    self.event_queue.clear();
    res
  }
}

#[cfg(test)]
mod tests {
  use super::super::exp::sentence::Sentence;
  use super::{MusicalTyperError, MusicalTyperEvent};

  struct KeyPress(f64, &'static str);

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

*3.0
[end]
"#,
      |config| config.ignore_invalid_properties(true),
    )?;

    let keypresses = &[KeyPress(2.22, "dakentesuto")];
    use MusicalTyperEvent::*;
    let expected_events = vec![
      PlayBgm("void.ogg".to_owned()),
      Pointed(10),
      Typed { mistaken: false },
      UpdateSentence(Sentence::new_with_inputted(
        "打鍵テスト",
        "だけんてすと",
        "d",
      )?),
      Pointed(10),
      Typed { mistaken: false },
      UpdateSentence(Sentence::new_with_inputted(
        "打鍵テスト",
        "だけんてすと",
        "da",
      )?),
      Pointed(10),
      Typed { mistaken: false },
      UpdateSentence(Sentence::new_with_inputted(
        "打鍵テスト",
        "だけんてすと",
        "dak",
      )?),
      Pointed(10),
      Typed { mistaken: false },
      UpdateSentence(Sentence::new_with_inputted(
        "打鍵テスト",
        "だけんてすと",
        "dake",
      )?),
      Pointed(10),
      Typed { mistaken: false },
      UpdateSentence(Sentence::new_with_inputted(
        "打鍵テスト",
        "だけんてすと",
        "daken",
      )?),
      Pointed(10),
      Typed { mistaken: false },
      UpdateSentence(Sentence::new_with_inputted(
        "打鍵テスト",
        "だけんてすと",
        "dakent",
      )?),
      Pointed(10),
      Typed { mistaken: false },
      UpdateSentence(Sentence::new_with_inputted(
        "打鍵テスト",
        "だけんてすと",
        "dakente",
      )?),
      Pointed(10),
      Typed { mistaken: false },
      UpdateSentence(Sentence::new_with_inputted(
        "打鍵テスト",
        "だけんてすと",
        "dakentes",
      )?),
      Pointed(10),
      Typed { mistaken: false },
      UpdateSentence(Sentence::new_with_inputted(
        "打鍵テスト",
        "だけんてすと",
        "dakentesu",
      )?),
      Pointed(10),
      Typed { mistaken: false },
      UpdateSentence(Sentence::new_with_inputted(
        "打鍵テスト",
        "だけんてすと",
        "dakentesut",
      )?),
      Pointed(10),
      Typed { mistaken: false },
      UpdateSentence(Sentence::new_with_inputted(
        "打鍵テスト",
        "だけんてすと",
        "dakentesuto",
      )?),
    ];

    let mut game =
      MusicalTyper::new(&test_score, MusicalTyperConfig::default())?;

    let mut actual_events = vec![];

    for KeyPress(time, key) in keypresses.iter() {
      actual_events.append(&mut game.elapse_time(*time));
      actual_events.append(&mut game.key_press(key.chars()));
    }

    for (expected, actual) in
      expected_events.iter().zip(actual_events.iter())
    {
      assert_eq!(expected, actual);
    }
    assert_eq!(expected_events.len(), actual_events.len());

    Ok(())
  }
}
