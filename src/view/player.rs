use sdl2::mixer::{Chunk, Music};

#[derive(Debug)]
pub enum PlayerError {
  AudioError(String),
}

use PlayerError::*;

pub struct Player<'music> {
  music: Option<Music<'music>>,
  chunks: Vec<Chunk>,
}

impl<'music> Drop for Player<'music> {
  fn drop(&mut self) {
    sdl2::mixer::Music::halt();
  }
}

impl<'music> Player<'music> {
  pub fn new() -> Self {
    Self {
      music: None,
      chunks: vec![],
    }
  }

  pub fn change_bgm(
    &mut self,
    bgm_name: &str,
  ) -> Result<(), PlayerError> {
    let bgm_file_path = format!("score/{}", bgm_name);
    let music = sdl2::mixer::Music::from_file(std::path::Path::new(
      &bgm_file_path,
    ))
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
}
