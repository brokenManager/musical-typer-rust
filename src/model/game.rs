use super::exp::{
  game_activity::GameActivity,
  note::NoteContent,
  scoremap::{
    lexer::ScoremapLexError, Scoremap, ScoremapError,
    ScoremapMetadata,
  },
  sentence::{roman::roman_lexer::RomanParseError, Sentence},
  time::Seconds,
};
use std::io::Error;
use MusicalTyperError::*;
use MusicalTyperEvent::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub enum MusicalTypeResult {
  Correct,
  Missed,
  Vacant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MusicalTyperEvent {
  PlayBgm(String),
  UpdateSentence(Sentence),
  MissedSentence(Sentence),
  CompletedSentence(Sentence),
  DidPerfectSection,
  Typed(MusicalTypeResult),
  EndOfScore,
}

#[derive(Debug)]
pub enum MusicalTyperError {
  SongDataNotFound,
  FileReadError { reason: String },
  ScoremapBuildError(ScoremapError),
}

impl From<Error> for MusicalTyperError {
  fn from(err: Error) -> Self {
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
  metadata: ScoremapMetadata,
  accumulated_time: Seconds,
  event_queue: Vec<MusicalTyperEvent>,
  config: MusicalTyperConfig,
}

impl MusicalTyper {
  pub fn new(
    score: Scoremap,
    config: MusicalTyperConfig,
  ) -> Result<Self, MusicalTyperError> {
    let mut event_queue = vec![];
    let activity = GameActivity::new(score.sections);

    let metadata = score.metadata;
    if let Some(song_data) = metadata.get("song_data") {
      event_queue.push(PlayBgm(song_data.into()));
    } else {
      return Err(SongDataNotFound);
    }

    Ok(MusicalTyper {
      activity,
      metadata,
      accumulated_time: 0.0.into(),
      event_queue,
      config,
    })
  }

  pub fn activity(&self) -> &GameActivity {
    &self.activity
  }

  #[must_use]
  pub fn key_press(
    &mut self,
    typed: impl Iterator<Item = char>,
  ) -> Vec<MusicalTyperEvent> {
    let prev_sentence = self.activity.current_sentence();
    let prev_completed = prev_sentence.completed();
    for typed in typed {
      use super::exp::scoremap::section::note::TypeResult::*;
      let result = self.activity.input(typed);
      match result {
        Succeed => {
          self.activity.point(self.config.correct_type as i32);
          self.event_queue.push(Typed(MusicalTypeResult::Correct));
        }
        Mistaken => {
          self.activity.point(-(self.config.wrong_type as i32));
          self.event_queue.push(Typed(MusicalTypeResult::Missed));
        }
        Vacant => {
          self.event_queue.push(Typed(MusicalTypeResult::Vacant));
        }
      }
    }
    let curr_sentence = self.activity.current_sentence();
    let curr_completed = curr_sentence.completed();

    let mut events = vec![];
    if !prev_completed && curr_completed {
      if let Some(true) = self
        .activity
        .current_section()
        .map(|section| 1.0 <= section.accuracy())
      {
        self.activity.point(self.config.perfect_section as i32);
        events.push(DidPerfectSection);
      }
      if let Some(true) = self
        .activity
        .current_note()
        .map(|note| 1.0 <= note.accuracy())
      {
        self.activity.point(self.config.perfect_sentence as i32);
      }
      self.activity.point(self.config.complete_sentence as i32);
      events.push(CompletedSentence(prev_sentence));
    }

    [self.pack_events(), events].concat()
  }

  #[must_use]
  pub fn elapse_time(
    &mut self,
    delta_time: Seconds,
  ) -> Vec<MusicalTyperEvent> {
    self.accumulated_time += delta_time;

    let completed = self.activity.current_sentence().completed();
    let prev_sentence = self.activity.current_sentence();
    let prev_note_id = self.activity.current_note_id();

    self.activity.update_time(self.accumulated_time.clone());

    let curr_note_id = self.activity.current_note_id();

    if curr_note_id == "" {
      return vec![EndOfScore];
    }

    let mut events = vec![];
    if !completed && (prev_note_id != curr_note_id) {
      self.activity.point(-(self.config.missed_sentence as i32));
      events.push(MissedSentence(prev_sentence));
    }

    [self.pack_events(), events].concat()
  }

  fn pack_events(&mut self) -> Vec<MusicalTyperEvent> {
    let sentence = self.activity.current_sentence();
    self.event_queue.push(UpdateSentence(sentence));

    let res = self.event_queue.iter().cloned().collect();
    self.event_queue.clear();
    res
  }

  pub fn accumulated_time(&self) -> Seconds {
    self.accumulated_time.clone()
  }

  pub fn section_remaining_ratio(&self) -> f64 {
    self.activity.remaining_ratio(self.accumulated_time.clone())
  }

  pub fn get_metadata(&'_ self, key: &str) -> String {
    match key {
      "title" => self
        .metadata
        .get("title")
        .cloned()
        .unwrap_or("曲名不詳".into()),
      "song_author" => self
        .metadata
        .get("song_author")
        .cloned()
        .unwrap_or("作曲者不詳".into()),
      _ => "".into(),
    }
  }
}
