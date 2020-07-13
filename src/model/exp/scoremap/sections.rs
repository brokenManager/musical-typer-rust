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
    let mut index_opt = None;
    for (index, section) in self.sections.iter_mut().enumerate() {
      if section.update(&time) {
        index_opt = Some(index);
        break;
      }
    }
    if let Some(index) = index_opt {
      self.current_section_index = index;
      self.current_section()
    } else {
      None
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = &Section> {
    self.sections.iter()
  }

  #[allow(dead_code)]
  pub fn len(&self) -> usize {
    self.sections.len()
  }
}
