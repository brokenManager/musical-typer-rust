use super::string_to_input::StringToInput;

pub type Seconds = f64;

pub type SectionId = String;

pub struct Section {
  from: NoteId,
  to: NoteId,
}

impl Section {
  pub fn new(from: NoteId, to: NoteId) -> Section {
    Section { from, to }
  }

  pub fn from(&self) -> &str {
    &self.from
  }

  pub fn to(&self) -> &str {
    &self.to
  }
}

#[derive(Debug)]
pub enum NoteContent {
  Sentence(StringToInput),
  Caption(String),
  Blank,
}

pub type NoteId = String;

#[derive(Debug)]
pub struct Note {
  id: NoteId,
  time: Seconds,
  content: NoteContent,
}

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

impl Note {
  fn new(time: f64, content: NoteContent) -> Self {
    let id =
      thread_rng().sample_iter(&Alphanumeric).take(5).collect();
    Note { id, time, content }
  }

  pub fn sentence(
    time: Seconds,
    origin: &str,
    yomigana: &str,
  ) -> Result<Self, String> {
    Ok(Self::new(
      time,
      NoteContent::Sentence(StringToInput::new(origin, yomigana)?),
    ))
  }

  pub fn caption(time: Seconds, caption: &str) -> Self {
    Self::new(time, NoteContent::Caption(caption.to_owned()))
  }

  pub fn blank(time: Seconds) -> Self {
    Self::new(time, NoteContent::Blank)
  }
}
