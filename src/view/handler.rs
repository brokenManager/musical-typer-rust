use sdl2::{event::Event, rect::Point, Sdl};

#[derive(Debug)]
pub enum HandleError {
  TimerError(String),
  EventError(String),
}

use HandleError::*;

#[readonly::make]
#[derive(Clone, PartialEq)]
pub struct MouseState {
  pub mouse_pos: Point,
  pub mouse_pressed: bool,
  pub started_pressing: Point,
  pub ended_pressing: Point,
}

#[derive(Clone)]
pub struct Handler {
  sdl: Sdl,
  mouse_state: MouseState,
}

impl Handler {
  pub fn new(sdl: Sdl) -> Self {
    Self {
      sdl,
      mouse_state: MouseState {
        mouse_pos: Point::new(0, 0),
        mouse_pressed: false,
        started_pressing: Point::new(0, 0),
        ended_pressing: Point::new(0, 0),
      },
    }
  }

  pub fn mouse_state(&self) -> &MouseState {
    &self.mouse_state
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
      use sdl2::event::Event::*;
      match &event {
        MouseMotion { x, y, .. } => {
          self.mouse_state.mouse_pos = Point::new(*x, *y);
        }
        MouseButtonDown {
          x, y, mouse_btn, ..
        } => {
          use sdl2::mouse::MouseButton::*;
          if let Left = mouse_btn {
            self.mouse_state.mouse_pressed = true;
            self.mouse_state.started_pressing = Point::new(*x, *y);
            self.mouse_state.ended_pressing = Point::new(0, 0);
          }
        }
        MouseButtonUp {
          x, y, mouse_btn, ..
        } => {
          use sdl2::mouse::MouseButton::*;
          if let Left = mouse_btn {
            self.mouse_state.mouse_pressed = false;
            self.mouse_state.ended_pressing = Point::new(*x, *y);
          }
        }
        _ => {}
      }
      f(event)
    }
    Ok(())
  }
}
