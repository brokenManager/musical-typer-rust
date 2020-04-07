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
  typed_key_buf: BTreeSet<char>,
  sentence: Option<Sentence>,
}

impl GameView {
  pub fn new(
    width: u32,
    height: u32,
    score: Scoremap,
  ) -> Result<Self, ViewError> {
    let ctx = sdl2::init()
      .map_err(|e| ViewError::InitError { message: e })?;

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
      typed_key_buf: BTreeSet::new(),
      sentence: None,
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

    let mut mt_events = vec![];

    'main: loop {
      let time = std::time::Instant::now();
      {
        for mt_event in mt_events.iter() {
          use MusicalTyperEvent::*;
          match mt_event {
            PlayBgm(bgm_name) => {}
            UpdateSentence(sentence) => {}
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
              self.typed_key_buf.insert(keycode_to_char(keycode));
            }
            KeyUp {
              keycode: Some(keycode),
              ..
            } => {
              self.typed_key_buf.remove(&keycode_to_char(keycode));
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
          pressed_keys: &self
            .typed_key_buf
            .iter()
            .cloned()
            .collect::<Vec<char>>()
            .as_slice(),
          sentence: &self.sentence,
        },
      )?;

      self.canvas.present();

      let typed_key_buf = self.typed_key_buf.clone();
      self.model.key_press(typed_key_buf.into_iter());

      let elapsed =
        time.elapsed().as_nanos() as f64 / 1_000_000_000.0;
      mt_events = self.model.elapse_time(elapsed);
      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
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
