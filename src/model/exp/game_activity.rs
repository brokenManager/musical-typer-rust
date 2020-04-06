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
  notes: BTreeMap<String, Note>,
  sections: Vec<Section>,
  current_section: Option<Section>,
}

impl GameActivity {
  pub fn new(notes: &Vec<Note>) -> Self {
    let shifted_notes = notes.iter().cloned().skip(1);
    let mut sections = notes
      .iter()
      .zip(shifted_notes)
      .map(|(note, next): (&Note, Note)| {
        Section::new(note.id(), note.time(), next.time())
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
      notes: notes_map,
      sections,
      current_section: None,
    }
  }

  pub fn current_section(&self) -> Option<Section> {
    self.current_section.clone()
  }

  fn current_note_mut(&mut self) -> Option<&mut Note> {
    self.current_section().and_then(move |section| {
      self.notes.get_mut(&section.foreign_note)
    })
  }

  pub fn current_note(&self) -> Option<&Note> {
    self
      .current_section()
      .and_then(|section| self.notes.get(&section.foreign_note))
  }

  pub fn accuracy(&self) -> f64 {
    let typed_count = self.succeed_count + self.mistyped_count;
    self.mistyped_count as f64 / typed_count as f64
  }

  pub fn update_time(&mut self, time: Seconds) {
    self.state = State::OnGame;
    for section in self.sections.iter() {
      if section.from <= time && time < section.to {
        self.current_section = Some(section.clone());
        return;
      }
    }
    self.current_section = None;
    self.state = State::GameOver;
  }

  pub fn input(&mut self, typed: char) {
    if let State::OnGame = self.state {
      return;
    }
    if let Some(note) = self.current_note_mut() {
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
    self.current_note().and_then(|note| {
      if let NoteContent::Sentence { sentence, .. } = note.content() {
        Some(sentence)
      } else {
        None
      }
    })
  }
}
