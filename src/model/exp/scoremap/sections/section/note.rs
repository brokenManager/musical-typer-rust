use scoring::Scoring;
use sentence::Sentence;

mod scoring;
pub mod sentence;

#[derive(Debug)]
pub enum TypeResult {
  Succeed,
  Mistaken,
  Vacant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoteContent {
  Sentence { sentence: Sentence, succeed: bool },
  Caption(String),
  Blank,
}

pub type NoteId = String;

#[derive(Debug, Clone)]
pub struct Note {
  id: NoteId,
  duration: Duration,
  content: NoteContent,
  scoring: Scoring,
}

use crate::model::exp::time::Duration;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

impl Note {
  fn new(duration: Duration, content: NoteContent) -> Self {
    let id = thread_rng()
      .sample_iter(&Alphanumeric)
      .take(5)
      .map(|n| n as char)
      .collect();
    Self {
      id,
      duration,
      content,
      scoring: Scoring::new(),
    }
  }

  pub fn sentence(duration: Duration, sentence: Sentence) -> Self {
    Self::new(
      duration,
      NoteContent::Sentence {
        sentence,
        succeed: false,
      },
    )
  }

  pub fn caption(duration: Duration, caption: &str) -> Self {
    Self::new(duration, NoteContent::Caption(caption.into()))
  }

  pub fn blank(duration: Duration) -> Self {
    Self::new(duration, NoteContent::Blank)
  }

  pub fn id(&self) -> NoteId {
    self.id.clone()
  }

  pub fn duration(&self) -> &Duration {
    &self.duration
  }

  pub fn input(&mut self, typed: char) -> TypeResult {
    use NoteContent::Sentence;
    use TypeResult::*;

    let res = if let Sentence {
      sentence, succeed, ..
    } = &mut self.content
    {
      if sentence.completed() {
        Vacant
      } else if sentence.input(typed) {
        *succeed = true;
        Succeed
      } else {
        Mistaken
      }
    } else {
      Vacant
    };
    self.scoring.point(&res);
    res
  }

  pub fn content(&self) -> &NoteContent {
    &self.content
  }

  pub fn accuracy(&self) -> f64 {
    match self.content {
      NoteContent::Sentence { .. } => {
        self.scoring.accuracy().as_f64()
      }
      _ => 1.0,
    }
  }
}
