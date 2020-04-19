use crate::model::exp::scoremap::Scoremap;
use crate::model::exp::{
  minute_second::Seconds, note::NoteContent, sentence::Sentence,
};
use crate::model::game::{
  MusicalTyper, MusicalTyperConfig, MusicalTyperEvent,
};

use sdl2::keyboard::Keycode;

use std::collections::BTreeSet;

mod whole;

use super::{handler::Handler, renderer::Renderer, ViewError};
use whole::WholeProps;

pub struct GameView<'renderer, 'ttf, 'canvas, 'handler, 'sdl> {
  width: u32,
  height: u32,
  renderer: &'renderer Renderer<'ttf, 'canvas>,
  handler: &'handler Handler<'sdl>,
  model: MusicalTyper,
  score: Scoremap,
}

impl<'renderer, 'ttf, 'canvas, 'handler, 'sdl>
  GameView<'renderer, 'ttf, 'canvas, 'handler, 'sdl>
{
  pub fn new(
    renderer: &'renderer Renderer<'ttf, 'canvas>,
    handler: &'handler Handler<'sdl>,
    score: Scoremap,
    width: u32,
    height: u32,
  ) -> Result<Self, ViewError> {
    Ok(GameView {
      width,
      height,
      renderer,
      handler,
      model: MusicalTyper::new(
        &score,
        MusicalTyperConfig::default(),
      )?,
      score,
    })
  }

  pub fn run<'a: 'ttf + 'canvas>(
    &'a mut self,
  ) -> Result<(), ViewError> {
    let all_roman_len =
      self.score.notes.iter().fold(0, |acc, note| {
        match note.content() {
          NoteContent::Sentence { sentence, .. } => {
            sentence.roman().will_input.len() + acc
          }
          _ => acc,
        }
      });

    struct TypeTimepoint(Seconds);

    let mut mt_events = vec![];
    let mut musics = vec![];
    let mut pressed_key_buf = BTreeSet::new();
    let mut typed_key_buf = vec![];
    let mut sentence: Option<Sentence> = None;
    let mut score_point = 0;
    let mut correction_type_count = 0;
    let mut wrong_type_count = 0;
    let mut timepoints = std::collections::VecDeque::new();

    'main: loop {
      let time = std::time::Instant::now();
      {
        for mt_event in mt_events.iter() {
          use MusicalTyperEvent::*;
          match mt_event {
            PlayBgm(bgm_name) => {
              let bgm_file_path = format!("score/{}", bgm_name);
              let music = sdl2::mixer::Music::from_file(
                std::path::Path::new(&bgm_file_path),
              )
              .map_err(|e| ViewError::AudioError { message: e })?;
              music
                .play(0)
                .map_err(|e| ViewError::AudioError { message: e })?;
              musics.push(music);
            }
            UpdateSentence(new_sentence) => {
              sentence = Some(new_sentence.clone());
            }
            Pointed(point) => {
              score_point += point;
            }
            Typed { mistaken } => {
              if *mistaken {
                wrong_type_count += 1;
              } else {
                correction_type_count += 1;
                timepoints.push_back(TypeTimepoint(
                  self.model.accumulated_time(),
                ));
              }
            }
          }
        }
      }
      {
        use sdl2::event::Event::*;
        let should_quit = false;
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
          break 'main;
        }
      }
      {
        let expire_limit = self.model.accumulated_time() - 5.0;
        while let Some(front) = timepoints.front() {
          if front.0 < expire_limit {
            timepoints.pop_front();
          } else {
            break;
          }
        }
      }

      whole::render(
        &mut self.renderer,
        sdl2::rect::Rect::new(0, 0, self.width, self.height),
        &WholeProps {
          pressed_keys: &pressed_key_buf
            .iter()
            .cloned()
            .collect::<Vec<char>>()
            .as_slice(),
          sentence: &sentence,
          title: self
            .score
            .metadata
            .get("title")
            .unwrap_or(&"曲名不詳".to_owned()),
          song_author: self
            .score
            .metadata
            .get("song_author")
            .unwrap_or(&"作曲者不詳".to_owned()),
          score_point,
        },
      )?;
      self.renderer.flush();

      let typed_key_buf_cloned = typed_key_buf.clone();
      typed_key_buf.clear();
      mt_events =
        self.model.key_press(typed_key_buf_cloned.into_iter());

      let draw_time = time.elapsed().as_secs_f64();

      self.handler.delay((1e3 / 60.0) as u32);

      let elapsed = time.elapsed().as_secs_f64();

      mt_events.append(&mut self.model.elapse_time(elapsed));
      print!(
        "\rFPS: {}, Playing: {}     ",
        1.0 / draw_time,
        sdl2::mixer::Music::is_playing()
      );
    }
    sdl2::mixer::Music::fade_out(500)
      .map_err(|e| ViewError::AudioError { message: e })?;
    self.handler.delay(505);
    sdl2::mixer::Music::halt();

    Ok(())
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
