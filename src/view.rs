use crate::model::exp::scoremap::Scoremap;
use crate::model::exp::sentence::Sentence;
use crate::model::game::{
  MusicalTyper, MusicalTyperConfig, MusicalTyperError,
  MusicalTyperEvent,
};

use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use std::collections::BTreeSet;
use std::time::Duration;

mod text;
mod whole;

use text::{TextBuilder, TextError};
use whole::WholeProps;

#[derive(Debug)]
pub enum ViewError {
  ModelError(MusicalTyperError),
  InitError { message: String },
  FontError { message: String },
  AudioError { message: String },
  TextError(TextError),
  RenderError(String),
}

impl From<MusicalTyperError> for ViewError {
  fn from(err: MusicalTyperError) -> Self {
    ViewError::ModelError(err)
  }
}

impl From<TextError> for ViewError {
  fn from(err: TextError) -> Self {
    match err {
      TextError::RenderError(e) => ViewError::RenderError(e),
      _ => ViewError::TextError(err),
    }
  }
}

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

    let _audio = ctx
      .audio()
      .map_err(|e| ViewError::AudioError { message: e })?;
    sdl2::mixer::open_audio(
      44100,
      sdl2::mixer::DEFAULT_FORMAT,
      sdl2::mixer::DEFAULT_CHANNELS,
      1024,
    )
    .map_err(|e| ViewError::AudioError { message: e })?;
    let _mixer = sdl2::mixer::init(
      sdl2::mixer::InitFlag::OGG | sdl2::mixer::InitFlag::MP3,
    )
    .map_err(|e| ViewError::AudioError { message: e })?;
    println!("linked version: {}", sdl2::mixer::get_linked_version());

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

    let mut timer = self
      .ctx
      .timer()
      .map_err(|e| ViewError::InitError { message: e })?;

    let mut mt_events = vec![];
    let mut musics = vec![];
    let mut typed_key_buf = BTreeSet::new();
    let mut sentence: Option<Sentence> = None;

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
                .fade_in(1, 30)
                .map_err(|e| ViewError::AudioError { message: e })?;
              musics.push(music);
            }
            UpdateSentence(new_sentence) => {
              sentence = Some(new_sentence.clone());
            }
            Pointed(point) => {}
            Typed { mistaken } => {}
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
              typed_key_buf.insert(keycode_to_char(keycode));
            }
            KeyUp {
              keycode: Some(keycode),
              ..
            } => {
              typed_key_buf.remove(&keycode_to_char(keycode));
            }
            _ => {}
          }
        }
      }
      whole::render(
        &mut self.canvas,
        sdl2::rect::Rect::new(0, 0, self.width, self.height),
        builder.clone(),
        &WholeProps {
          pressed_keys: &typed_key_buf
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
        },
      )?;

      self.canvas.present();

      let typed_key_buf = typed_key_buf.clone();
      mt_events = self.model.key_press(typed_key_buf.into_iter());

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
