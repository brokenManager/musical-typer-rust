use crate::model::exp::{
  minute_second::Seconds, note::NoteContent, scoremap::Scoremap,
  sentence::Sentence,
};
use crate::model::game::{
  MusicalTyper, MusicalTyperConfig, MusicalTyperEvent,
};

use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use std::collections::BTreeSet;

mod whole;

use super::{text::TextBuilder, ViewError};
use whole::WholeProps;

pub struct GameView {
  width: u32,
  height: u32,
  ctx: Sdl,
  canvas: Canvas<Window>,
  model: MusicalTyper,
  score: Scoremap,
}

impl GameView {
  pub fn new(
    width: u32,
    height: u32,
    score: Scoremap,
  ) -> Result<Self, ViewError> {
    let ctx = sdl2::init()
      .map_err(|e| ViewError::InitError { message: e })?;

    sdl2::mixer::open_audio(
      44100,
      sdl2::mixer::DEFAULT_FORMAT,
      sdl2::mixer::DEFAULT_CHANNELS,
      1024,
    )
    .map_err(|e| ViewError::AudioError { message: e })?;

    let video = ctx
      .video()
      .map_err(|e| ViewError::InitError { message: e })?;
    let window = video
      .window("Musical Typer", width, height)
      .position_centered()
      .opengl()
      .build()
      .map_err(|e| ViewError::InitError {
        message: e.to_string(),
      })?;

    let mut canvas = window.into_canvas().build().map_err(|e| {
      ViewError::InitError {
        message: e.to_string(),
      }
    })?;
    canvas.clear();
    canvas.present();

    Ok(GameView {
      width,
      height,
      ctx,
      canvas,
      model: MusicalTyper::new(
        &score,
        MusicalTyperConfig::default(),
      )?,
      score,
    })
  }

  pub fn run(&mut self) -> Result<(), ViewError> {
    let texture_creator = self.canvas.texture_creator();

    let ttf =
      sdl2::ttf::init().map_err(|e| ViewError::InitError {
        message: e.to_string(),
      })?;
    let font = ttf
      .load_font(
        std::path::Path::new("./asset/mplus-1m-medium.ttf"),
        128,
      )
      .map_err(|e| ViewError::FontError {
        message: e.to_string(),
      })?;

    let builder = TextBuilder::new(&font, &texture_creator);

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

    let mut timer = self
      .ctx
      .timer()
      .map_err(|e| ViewError::InitError { message: e })?;

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
        let mut poller = self.ctx.event_pump().map_err(|e| {
          ViewError::InitError {
            message: e.to_string(),
          }
        })?;
        for event in poller.poll_iter() {
          use sdl2::event::Event::*;
          match event {
            Quit { .. } => break 'main,
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
          }
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

      let accuracy = if correction_type_count == 0 {
        0.0
      } else {
        correction_type_count as f64
          / (correction_type_count + wrong_type_count) as f64
      };
      let achievement_rate =
        correction_type_count as f64 / all_roman_len as f64;
      let type_per_second = timepoints.len() as f64 / 5.0;

      whole::build(
        sdl2::rect::Rect::new(0, 0, self.width, self.height),
        builder.clone(),
        WholeProps {
          pressed_keys: pressed_key_buf
            .iter()
            .cloned()
            .collect::<Vec<char>>(),
          sentence: sentence.clone(),
          title: self
            .score
            .metadata
            .get("title")
            .cloned()
            .unwrap_or("曲名不詳".to_owned()),
          song_author: self
            .score
            .metadata
            .get("song_author")
            .cloned()
            .unwrap_or("作曲者不詳".to_owned()),
          score_point,
          accuracy,
          achievement_rate,
          type_per_second,
        },
      )?(&mut self.canvas)?;
      self.canvas.present();

      let typed_key_buf_cloned = typed_key_buf.clone();
      typed_key_buf.clear();
      mt_events =
        self.model.key_press(typed_key_buf_cloned.into_iter());

      timer.delay((1e3 / 60.0) as u32);

      let elapsed = time.elapsed().as_secs_f64();
      mt_events.append(&mut self.model.elapse_time(elapsed));
      print!(
        "\rFPS: {}, Playing: {}     ",
        1.0 / elapsed,
        sdl2::mixer::Music::is_playing()
      );
    }
    sdl2::mixer::Music::fade_out(500)
      .map_err(|e| ViewError::AudioError { message: e })?;
    timer.delay(505);
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
