use sdl2::mixer::{Channel, Chunk, Music};
use std::{collections::HashMap, path::Path};
use PlayerError::*;

pub enum SEKind {
  Correct,
  Fail,
  Vacant,
  GameOver,
  MissedSentence,
  PerfectSentence,
  PerfectSection,
}

#[derive(Debug)]
pub enum PlayerError {
  AudioError(String),
  FileError(std::io::Error),
}

impl From<std::io::Error> for PlayerError {
  fn from(err: std::io::Error) -> Self {
    FileError(err)
  }
}

type Chunks = HashMap<String, Chunk>;

pub struct Player<'music> {
  music: Option<Music<'music>>,
  chunks: Chunks,
}

impl<'music> Drop for Player<'music> {
  fn drop(&mut self) {
    sdl2::mixer::Music::halt();
  }
}

fn loaad_chunks() -> Result<Chunks, PlayerError> {
  let path = Path::new("asset/");
  let mut chunks: Chunks = HashMap::new();
  for entry in path.read_dir()? {
    let file = entry?;
    if let Some(ext) = file.path().extension() {
      if ext == "wav" {
        chunks.insert(
          file.path().file_stem().map_or("".into(), |name| {
            name.to_string_lossy().to_string()
          }),
          Chunk::from_file(file.path()).map_err(|e| AudioError(e))?,
        );
      }
    }
  }
  Channel::all().set_volume(112); // the max is 128
  Ok(chunks)
}

impl<'music> Player<'music> {
  pub fn new() -> Self {
    Self {
      music: None,
      chunks: loaad_chunks().expect("missing audio file dir"),
    }
  }

  pub fn change_bgm(
    &mut self,
    bgm_name: &str,
  ) -> Result<(), PlayerError> {
    let bgm_file_path = format!("score/{}", bgm_name);
    let music =
      sdl2::mixer::Music::from_file(Path::new(&bgm_file_path))
        .map_err(|e| AudioError(e))?;
    self.music = Some(music);
    self.play_bgm()?;
    Ok(())
  }

  pub fn play_bgm(&self) -> Result<(), PlayerError> {
    if let Some(ref music) = self.music {
      music.play(0).map_err(|e| AudioError(e))?;
    }
    Ok(())
  }

  pub fn stop_bgm(&self, fade_time: i32) -> Result<(), PlayerError> {
    sdl2::mixer::Music::fade_out(fade_time).map_err(|e| AudioError(e))
  }

  fn play_se_file(&self, name: &str) -> Result<(), PlayerError> {
    let chunk = self.chunks.get(name).ok_or(AudioError(format!(
      "missing such audio file: {}",
      name
    )))?;
    let _ = Channel::all().play(chunk, 0).map_err(|e| {
      eprintln!("{:?}", e);
    });
    Ok(())
  }

  pub fn play_se(&self, kind: SEKind) -> Result<(), PlayerError> {
    match kind {
      SEKind::Correct => self.play_se_file("correct"),
      SEKind::Fail => self.play_se_file("fail"),
      SEKind::Vacant => self.play_se_file("vacant"),
      SEKind::GameOver => self.play_se_file("gameover"),
      SEKind::MissedSentence => self.play_se_file("missed"),
      SEKind::PerfectSentence => {
        self.play_se_file("perfect_sentence")
      }
      SEKind::PerfectSection => self.play_se_file("perfect_section"),
    }
  }
}
