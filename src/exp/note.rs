use super::string_to_input::StringToInput;

pub type Seconds = f64;

pub type SectionId = String;

pub struct Section {
  id: SectionId,
  start: Seconds,
  end: Seconds,
}

impl Section {
  pub fn new(id: SectionId, start: Seconds, end: Seconds) -> Section {
    Section { id, start, end }
  }
}

pub enum NoteContent {
  Sentence(StringToInput),
  Caption(String),
  Blank,
}

pub type NoteId = String;

pub struct Note {
  id: NoteId,
  time: f64,
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

  pub fn sentence(time: f64, lyrics: &str) -> Result<Self, String> {
    Ok(Self::new(
      time,
      NoteContent::Sentence(StringToInput::new(lyrics)?),
    ))
  }

  pub fn caption(time: f64, caption: &str) -> Self {
    Self::new(time, NoteContent::Caption(caption.to_owned()))
  }
}
