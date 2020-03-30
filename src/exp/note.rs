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

pub enum Note {
  Sentence {
    section: Section,
    is_ended: bool,
    sentence: StringToInput,
  },
  Caption {
    section: Section,
    is_ended: bool,
    caption: String,
  },
}

impl Note {
  pub fn sentence(
    section: Section,
    lyrics: &str,
  ) -> Result<Note, String> {
    Ok(Note::Sentence {
      section,
      sentence: StringToInput::new(lyrics)?,
      is_ended: false,
    })
  }

  pub fn caption(section: Section, caption: &str) -> Note {
    Note::Caption {
      section,
      is_ended: false,
      caption: caption.to_owned(),
    }
  }
}
