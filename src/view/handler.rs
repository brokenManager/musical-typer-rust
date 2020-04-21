use sdl2::{event::Event, Sdl};

#[derive(Debug)]
pub enum HandleError {
  TimerError(String),
  EventError(String),
}

use HandleError::*;

pub struct Handler<'sdl> {
  sdl: &'sdl Sdl,
}

impl<'sdl> Handler<'sdl> {
  pub fn new(sdl: &'sdl Sdl) -> Self {
    Self { sdl }
  }

  pub fn delay(&self, ms: u32) -> Result<(), HandleError> {
    self.sdl.timer().map_err(|e| TimerError(e))?.delay(ms);
    Ok(())
  }

  pub fn poll_events<F>(
    &mut self,
    mut f: F,
  ) -> Result<(), HandleError>
  where
    F: FnMut(Event),
  {
    let mut poller = self
      .sdl
      .event_pump()
      .map_err(|e| EventError(e.to_string()))?;
    for event in poller.poll_iter() {
      f(event)
    }
    Ok(())
  }
}
