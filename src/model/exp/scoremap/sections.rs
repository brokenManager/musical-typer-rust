use crate::model::exp::time::Seconds;
use section::{
  note::{Note, TypeResult},
  Section,
};

pub mod section;

#[derive(Debug, Clone)]
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
