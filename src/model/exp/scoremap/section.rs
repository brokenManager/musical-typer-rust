use crate::model::exp::time::{Duration, Seconds};
use note::{Note, NoteContent, NoteId, TypeResult};

pub mod note;

#[derive(Debug)]
pub struct Section {
  notes: Vec<Note>,
  current_note_index: usize,
  duration: Duration,
}

impl Section {
  fn new(notes: Vec<Note>, duration: Duration) -> Self {
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

#[derive(Debug)]
pub struct Sections {
  sections: Vec<Section>,
  current_section_index: usize,
}

impl Sections {
  pub fn new(notes: Vec<Vec<Note>>) -> Self {
    let sections: Vec<_> = notes
      .into_iter()
      .map(|section| {
        let first = section.first().unwrap().duration();
        let last = section.last().unwrap().duration();
        Section::new(section.clone(), first.concat(last))
      })
      .collect();
    Self {
      sections,
      current_section_index: 0,
    }
  }

  pub fn current_section(&self) -> Option<&Section> {
    self.sections.get(self.current_section_index)
  }

  pub fn input(&mut self, typed: char) -> TypeResult {
    use TypeResult::*;
    if let Some(section) =
      self.sections.get_mut(self.current_section_index)
    {
      section.input(typed)
    } else {
      Vacant
    }
  }

  pub fn update(&mut self, time: Seconds) -> Option<&Section> {
    for (index, section) in self.sections.iter_mut().enumerate() {
      if section.update(&time) {
        self.current_section_index = index;
        break;
      }
    }
    self.current_section()
  }

  pub fn iter(&self) -> impl Iterator<Item = &Section> {
    self.sections.iter()
  }

  pub fn len(&self) -> usize {
    self.sections.len()
  }
}
