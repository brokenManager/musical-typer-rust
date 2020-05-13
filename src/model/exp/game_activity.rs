use super::{
  scoremap::section::{
    note::{
      sentence::Sentence, Note, NoteContent, NoteId, TypeResult,
    },
    Section, Sections,
  },
  time::Seconds,
};

enum State {
  BeforeStart,
  OnGame,
  GameOver,
}

pub struct GameActivity {
  state: State,
  sections: Sections,
}

impl GameActivity {
  pub fn new(sections: Sections) -> Self {
    let mut res = GameActivity {
      state: State::BeforeStart,
      sections,
    };
    res.update_time(0.0.into());
    res
  }

  pub fn current_section(&self) -> Option<&Section> {
    self.sections.current_section()
  }

  pub fn current_note(&self) -> Option<&Note> {
    self
      .sections
      .current_section()
      .map(|section| section.current_note())
  }

  pub fn update_time(&mut self, time: Seconds) {
    self.state = State::OnGame;
    if let Some(_) = self.sections.update(time) {
      return;
    }
    self.state = State::GameOver;
  }

  pub fn input(&mut self, typed: char) -> TypeResult {
    use TypeResult::*;
    if let State::OnGame = self.state {
      self.sections.input(typed)
    } else {
      Vacant
    }
  }

  pub fn current_sentence(&self) -> Sentence {
    self
      .current_note()
      .and_then(|note| {
        if let NoteContent::Sentence { sentence, .. } = note.content()
        {
          Some(sentence.clone())
        } else {
          None
        }
      })
      .unwrap_or(Sentence::empty())
  }

  pub fn current_note_id(&self) -> NoteId {
    self.current_section().map_or("".into(), |s| s.id())
  }

  pub fn remaining_ratio(&self, time: Seconds) -> f64 {
    self
      .current_note()
      .as_ref()
      .map_or(1.0, |note| note.duration().remaining_ratio(time))
  }

  pub fn sections(&self) -> &Sections {
    &self.sections
  }
}
