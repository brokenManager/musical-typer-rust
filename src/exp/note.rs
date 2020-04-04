use super::sentence::Sentence;

pub type Seconds = f64;

pub type SectionId = String;

#[derive(Debug, Clone)]
pub struct Section {
  pub foreign_note: NoteId,
  pub from: Seconds,
  pub to: Seconds,
}

impl Section {
  pub fn new(
    foreign_note: NoteId,
    from: Seconds,
    to: Seconds,
  ) -> Self {
    Section {
      foreign_note,
      from,
      to,
    }
  }
}

#[derive(Debug, Clone)]
pub enum NoteContent {
  Sentence { sentence: Sentence, succeed: bool },
  Caption(String),
  Blank,
}

pub type NoteId = String;

#[derive(Debug, Clone)]
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
      NoteContent::Sentence {
        sentence: Sentence::new(origin, yomigana)?,
        succeed: false,
      },
    ))
  }

  pub fn caption(time: Seconds, caption: &str) -> Self {
    Self::new(time, NoteContent::Caption(caption.to_owned()))
  }

  pub fn blank(time: Seconds) -> Self {
    Self::new(time, NoteContent::Blank)
  }

  pub fn id(&self) -> NoteId {
    self.id.clone()
  }

  pub fn time(&self) -> Seconds {
    self.time
  }

  pub fn input(&mut self, typed: char) {
    use NoteContent::Sentence;

    if let Sentence { sentence, succeed } = &mut self.content {
      if sentence.input(typed) {
        *succeed = true;
      }
    }
  }
}
