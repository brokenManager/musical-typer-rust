use std::collections::BTreeMap;

use super::note::{Note, Seconds, Section};

enum State {
  BeforeStart,
  OnGame,
  GameOver,
}

pub struct GameActivity {
  state: State,
  typed_count: u32,
  mistyped_count: u32,
  typing_section_index: usize,
  notes: BTreeMap<String, Note>,
  sections: Vec<Section>,
}

impl GameActivity {
  pub fn new(notes: &Vec<Note>) -> Self {
    let shifted_notes = notes.iter().cloned().skip(1);
    let sections = notes
      .iter()
      .zip(shifted_notes)
      .map(|(prev, note): (&Note, Note)| {
        Section::new(prev.id(), prev.time(), note.time())
      })
      .collect();
    let mut notes_map = BTreeMap::<String, Note>::new();
    for note in notes {
      notes_map.insert(note.id(), note.clone());
    }
    GameActivity {
      state: State::BeforeStart,
      typed_count: 0,
      mistyped_count: 0,
      typing_section_index: 0,
      notes: notes_map,
      sections,
    }
  }

  pub fn current_section(&self) -> Option<Section> {
    if self.typing_section_index < self.sections.len() {
      Some(self.sections[self.typing_section_index].clone())
    } else {
      None
    }
  }

  pub fn accuracy(&self) -> f64 {
    self.mistyped_count as f64 / self.typed_count as f64
  }

  pub fn update_time(&mut self, time: Seconds) {
    self.state = State::OnGame;
    for (i, section) in self.sections.iter().enumerate() {
      if section.from <= time && time <= section.to {
        self.typing_section_index = i;
        return;
      }
    }
    self.state = State::GameOver;
  }

  pub fn input(&mut self, typed: char) {
    if let State::OnGame = self.state {
      return;
    }
    let note_id =
      &self.sections[self.typing_section_index].foreign_note;
    if let Some(note) = self.notes.get_mut(note_id) {
      note.input(typed);
    }
  }
}
