use crate::model::{
  exp::{scoremap::Scoremap, sentence::Sentence, time::Seconds},
  game::{
    MusicalTypeResult, MusicalTyper, MusicalTyperConfig,
    MusicalTyperEvent,
  },
};

use sdl2::keyboard::Keycode;

use std::{
  collections::{BTreeSet, VecDeque},
  time::Instant,
};

mod whole;

use super::{
  handler::Handler,
  player::{Player, SEKind},
  renderer::{Component, RenderCtx},
  View, ViewError, ViewRoute,
};
use whole::{Whole, WholeProps};

pub struct GameView<'ttf, 'canvas> {
  renderer: RenderCtx<'ttf, 'canvas>,
  handler: Handler,
  model: MusicalTyper,
}

impl<'ttf, 'canvas> GameView<'ttf, 'canvas> {
  pub fn new(
    renderer: RenderCtx<'ttf, 'canvas>,
    handler: Handler,
    score: Scoremap,
  ) -> Result<Self, ViewError> {
    Ok(GameView {
      renderer,
      handler,
      model: MusicalTyper::new(score, MusicalTyperConfig::default())?,
    })
  }
}

impl<'ttf, 'canvas> View for GameView<'ttf, 'canvas> {
  fn run(&mut self) -> Result<ViewRoute, ViewError> {
    struct TypeTimepoint(Seconds);

    let mut mt_events = vec![];
    let mut player = Player::new();
    let mut pressed_key_buf = BTreeSet::new();
    let mut typed_key_buf = vec![];
    let mut sentence = Sentence::empty();
    let mut timepoints = VecDeque::new();
    let mut ended = None;

    loop {
      let time = Instant::now();
      {
        for mt_event in mt_events.iter() {
          use MusicalTyperEvent::*;
          match mt_event {
            PlayBgm(bgm_name) => {
              player.change_bgm(bgm_name)?;
            }
            UpdateSentence(new_sentence) => {
              sentence = new_sentence.clone();
            }
            Typed(result) => match result {
              MusicalTypeResult::Missed => {
                player.play_se(SEKind::Fail)?;
              }
              MusicalTypeResult::Correct => {
                timepoints.push_back(TypeTimepoint(
                  self.model.accumulated_time(),
                ));
                player.play_se(SEKind::Correct)?;
              }
              MusicalTypeResult::Vacant => {
                player.play_se(SEKind::Vacant)?;
              }
            },
            MissedSentence(_sentence) => {
              player.play_se(SEKind::MissedSentence)?;
              // TODO: Queue a missed animation
            }
            CompletedSentence(_sentence) => {
              player.play_se(SEKind::PerfectSentence)?;
              // TODO: Queue a completed animation
            }
            DidPerfectSection => {
              player.play_se(SEKind::PerfectSection)?;
              // TODO: Queue a perfect animation
            }
            EndOfScore => {
              if let None = ended {
                ended =
                  Some(self.model.accumulated_time() + 2.0.into());
              }
            }
          }
        }
      }
      {
        use sdl2::event::Event::*;
        let mut should_quit = false;
        self.handler.poll_events(|event| match event {
          Quit { .. } => {
            should_quit = true;
          }
          KeyDown {
            keycode: Some(keycode),
            ..
          } => {
            let key = keycode_to_char(keycode);
            if pressed_key_buf.insert(key) {
              typed_key_buf.push(key);
            }
          }
          KeyUp {
            keycode: Some(keycode),
            ..
          } => {
            pressed_key_buf.remove(&keycode_to_char(keycode));
          }
          _ => {}
        })?;
        if should_quit {
          player.stop_bgm(500)?;
          player.play_se(SEKind::GameOver)?;
          self.handler.delay(2500)?;
          return Ok(ViewRoute::Quit);
        }
      }
      {
        let expire_limit = self.model.accumulated_time() - 5.0.into();
        while let Some(front) = timepoints.front() {
          if front.0 < expire_limit {
            timepoints.pop_front();
          } else {
            break;
          }
        }
      }

      let type_per_second = timepoints.len() as f64 / 5.0;
      let client = sdl2::rect::Rect::new(
        0,
        0,
        self.renderer.borrow().width(),
        self.renderer.borrow().height(),
      );

      Whole::new(
        WholeProps {
          pressed_keys: pressed_key_buf.iter().cloned().collect(),
          sentence: &sentence,
          music_info: self.model.music_info(),
          type_per_second,
          score: self.model.activity().score(),
          section_remaining_ratio: self
            .model
            .section_remaining_ratio(),
        },
        client,
      )
      .render(&mut self.renderer.borrow_mut())?;

      self.renderer.borrow_mut().flush();

      let typed_key_buf_cloned = typed_key_buf.clone();
      typed_key_buf.clear();
      mt_events =
        self.model.key_press(typed_key_buf_cloned.into_iter());

      let draw_time = time.elapsed().as_secs_f64();

      self
        .handler
        .delay((1e3 / 60.0 - draw_time * 1e3).max(0.0) as u32)?;

      let elapsed = time.elapsed().as_secs_f64();

      mt_events.append(&mut self.model.elapse_time(elapsed.into()));
      print!(
        "\rFPS: {}, Playing: {}     ",
        1.0 / draw_time,
        sdl2::mixer::Music::is_playing()
      );

      if ended
        .as_ref()
        .map_or(false, |ended| ended < &self.model.accumulated_time())
      {
        return Ok(ViewRoute::ResultView(
          self.model.activity().score(),
          self.model.music_info(),
        ));
      }
    }
  }
}

fn keycode_to_char(keycode: Keycode) -> char {
  use Keycode::*;
  match keycode {
    A => 'a',
    B => 'b',
    C => 'c',
    D => 'd',
    E => 'e',
    F => 'f',
    G => 'g',
    H => 'h',
    I => 'i',
    J => 'j',
    K => 'k',
    L => 'l',
    M => 'm',
    N => 'n',
    O => 'o',
    P => 'p',
    Q => 'q',
    R => 'r',
    S => 's',
    T => 't',
    U => 'u',
    V => 'v',
    W => 'w',
    X => 'x',
    Y => 'y',
    Z => 'z',
    Minus => '-',
    _ => '\0',
  }
}
