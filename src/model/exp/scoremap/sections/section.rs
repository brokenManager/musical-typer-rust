use crate::model::exp::time::{Duration, Seconds};
use note::{Note, NoteContent, NoteId, TypeResult};

pub mod note;

#[derive(Debug, Clone)]
pub struct Section {
  notes: Vec<Note>,
  current_note_index: usize,
  duration: Duration,
}

impl Section {
  pub fn new(notes: Vec<Note>, duration: Duration) -> Self {
    Self {
      notes,
      current_note_index: 0,
      duration,
    }
  }

  pub fn current_note(&self) -> &Note {
    &self.notes[self.current_note_index]
  }

  pub fn id(&self) -> NoteId {
    self.current_note().id()
  }

  pub fn input(&mut self, typed: char) -> TypeResult {
    self.notes[self.current_note_index].input(typed)
  }

  pub fn remaining_ratio(&self, now: Seconds) -> f64 {
    self.duration.remaining_ratio(now)
  }

  fn char_count(&self) -> (u32, u32) {
    use NoteContent::*;
    self.notes.iter().fold((0, 0), |curr, note| {
      let (will_input, inputted) = match note.content() {
        Sentence { sentence, .. } => {
          let roman = sentence.roman();
          (roman.will_input.len() as u32, roman.inputted.len() as u32)
        }
        Caption(_) => (0, 0),
        Blank => (0, 0),
      };
      (curr.0 + will_input, curr.1 + inputted)
    })
  }

  pub fn progress(&self) -> f64 {
    let (will_input, inputted) = self.char_count();
    inputted as f64 / (will_input + inputted) as f64
  }

  pub fn accuracy(&self) -> f64 {
    let mut accuracies: Vec<_> =
      self.notes.iter().map(|note| note.accuracy()).collect();
    accuracies.sort_by(|ref a, ref b| a.partial_cmp(b).unwrap());

    if accuracies.len() % 2 == 0 {
      accuracies[accuracies.len() / 2]
    } else {
      (accuracies[accuracies.len() / 2 - 1]
        + accuracies[accuracies.len() / 2])
        / 2.0
    }
  }

  pub fn update(&mut self, time: &Seconds) -> bool {
    for (index, note) in self.notes.iter().enumerate() {
      if note.duration().includes(&time) {
        self.current_note_index = index;
        return true;
      }
    }
    false
  }

  pub fn iter(&self) -> impl Iterator<Item = &Note> {
    self.notes.iter()
  }

  pub fn len(&self) -> usize {
    self.notes.len()
  }
}
