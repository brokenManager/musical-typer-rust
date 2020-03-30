use super::string_to_input::StringToInput;

pub type Seconds = f64;

pub enum Note {
  Sentence {
    time: Seconds,
    is_ended: bool,
    sentence: StringToInput,
  },
  Caption {
    time: Seconds,
    is_ended: bool,
    caption: String,
  },
}

impl Note {
  pub fn sentence(
    length: Seconds,
    lyrics: &str,
  ) -> Result<Note, String> {
    Ok(Note::Sentence {
      time: length,
      sentence: StringToInput::new(lyrics)?,
      is_ended: false,
    })
  }

  pub fn caption(length: Seconds, caption: &str) -> Note {
    Note::Caption {
      time: length,
      is_ended: false,
      caption: caption.to_owned(),
    }
  }
}

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
