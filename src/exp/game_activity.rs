use std::collections::BTreeMap;

use super::minute_second::Seconds;
use super::note::{Note, NoteContent, Section, TypeResult};
use super::sentence::Sentence;

enum State {
  BeforeStart,
  OnGame,
  GameOver,
}

pub struct GameActivity {
  state: State,
  succeed_count: u32,
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
      succeed_count: 0,
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
    let typed_count = self.succeed_count + self.mistyped_count;
    self.mistyped_count as f64 / typed_count as f64
  }

  pub fn update_time(&mut self, time: Seconds) {
    self.state = State::OnGame;
    for (i, section) in self
      .sections
      .iter()
      .skip(self.typing_section_index)
      .enumerate()
    {
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
      use TypeResult::*;
      match note.input(typed) {
        Succeed => {
          self.succeed_count += 1;
        }
        Mistaken => {
          self.mistyped_count += 1;
        }
        Vacant => {}
      }
    }
  }

  pub fn current_sentence(&self) -> Option<&Sentence> {
    let note_id =
      &self.sections[self.typing_section_index].foreign_note;
    self.notes.get(note_id).and_then(|note| {
      if let NoteContent::Sentence { sentence, .. } = note.content() {
        Some(sentence)
      } else {
        None
      }
    })
  }
}
