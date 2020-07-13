use super::{
  scoremap::sections::{
    section::{
      note::{
        sentence::Sentence, Note, NoteContent, NoteId, TypeResult,
      },
      Section,
    },
    Sections,
  },
  time::Seconds,
};

enum State {
  BeforeStart,
  OnGame,
  GameOver,
}

#[readonly::make]
#[derive(Clone, PartialEq)]
pub struct GameScore {
  pub score_point: i32,
  pub achievement_rate: f64,
  pub accuracy: f64,
  correction_type_count: u32,
  wrong_type_count: u32,
  all_roman_len: usize,
}

impl GameScore {
  pub fn new(
    score_point: i32,
    achievement_rate: f64,
    accuracy: f64,
  ) -> Self {
    Self {
      score_point,
      achievement_rate,
      accuracy,
      correction_type_count: 0,
      wrong_type_count: 0,
      all_roman_len: 0,
    }
  }

  fn update(&mut self, type_result: &TypeResult) {
    match type_result {
      TypeResult::Mistaken => {
        self.wrong_type_count += 1;
      }
      TypeResult::Succeed => {
        self.correction_type_count += 1;
      }
      _ => return,
    };

    self.achievement_rate = (self.correction_type_count as f64
      / self.all_roman_len as f64)
      .min(1.);
    self.accuracy = if self.correction_type_count == 0 {
      0.0
    } else {
      self.correction_type_count as f64
        / (self.correction_type_count + self.wrong_type_count) as f64
    };
  }
}

pub struct GameActivity {
  state: State,
  sections: Sections,
  score: GameScore,
}

impl GameActivity {
  pub fn new(sections: Sections) -> Self {
    let all_roman_len = sections.iter().fold(0, |acc, section| {
      section.iter().fold(0, |acc, note| match note.content() {
        NoteContent::Sentence { sentence, .. } => {
          sentence.roman().will_input.len() + acc
        }
        _ => acc,
      }) + acc
    });
    let mut res = GameActivity {
      state: State::BeforeStart,
      sections,
      score: GameScore {
        score_point: 0,
        achievement_rate: 0.0,
        accuracy: 0.0,
        correction_type_count: 0,
        wrong_type_count: 0,
        all_roman_len,
      },
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
    let res = if let State::OnGame = self.state {
      self.sections.input(typed)
    } else {
      Vacant
    };
    self.score.update(&res);
    res
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

  pub fn point(&mut self, amount: i32) {
    self.score.score_point += amount;
  }

  pub fn score(&self) -> GameScore {
    self.score.clone()
  }

  pub fn is_game_over(&self) -> bool {
    if let State::GameOver = self.state {
      true
    } else {
      false
    }
  }
}
