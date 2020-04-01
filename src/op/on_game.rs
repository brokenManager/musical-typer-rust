use crate::exp::game_stat::GameActivity;
use crate::exp::note::{Note, Section};
use crate::exp::scoremap::Scoremap;
use crate::exp::string_to_input::StringToInput;

pub trait Controller {
  fn key_press(&mut self) -> char;
  fn elapse_time(&mut self) -> f64;
}
pub trait Presenter {
  fn play_bgm(&mut self, name: &str);
  fn decrease_remaining_time(&mut self, delta_time: f64);
  fn update_string_to_input(&mut self, string: &StringToInput);
  fn mistyped(&mut self);
  fn flush_screen(&mut self);
}

pub struct MusicalTyper {
  activity: GameActivity,
}

impl MusicalTyper {
  pub fn new(score: Scoremap) -> Self {
    let notes = score.notes();
    let shifted_notes = notes.iter().cloned().skip(1);
    let sections = notes
      .iter()
      .zip(shifted_notes)
      .map(|(prev, note): (&Note, Note)| {
        Section::new(prev.id(), note.id())
      })
      .collect();
    MusicalTyper {
      activity: GameActivity::new(sections),
    }
  }

  pub fn run_game(
    &mut self,
    controller: &mut impl Controller,
    presenter: &mut impl Presenter,
  ) -> Result<(), String> {
    Ok(())
  }
}
