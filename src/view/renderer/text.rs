use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::{surface::Surface, ttf::Font};

#[derive(Debug)]
pub enum TextError {
  FontError(sdl2::ttf::FontError),
  TextureError(sdl2::render::TextureValueError),
}

pub struct Text<'surface> {
  surface: Surface<'surface>,
  aspect: f64,
}

impl<'surface> Text<'surface> {
  pub fn new(
    style: &TextStyle,
    font: &Font,
  ) -> Result<Self, TextError> {
    let TextStyle { text, color, .. } = style;
    let aspect = {
      let (w, h) =
        font.size_of(text).map_err(|e| TextError::FontError(e))?;
      w as f64 / h as f64
    };
    let text = if text == "" { " " } else { &text };
    let surface = font
      .render(text)
      .blended(color.clone())
      .map_err(|e| TextError::FontError(e))?;

    Ok(Self { surface, aspect })
  }

  pub fn surface(&self) -> &Surface {
    &self.surface
  }

  pub fn aspect(&self) -> f64 {
    self.aspect
  }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum TextAlign {
  Left,
  Center,
  Right,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TextStyle {
  text: String,
  color: Color,
  line_height: u32,
  align: TextAlign,
  pos: Point,
}

impl TextStyle {
  pub fn new() -> Self {
    TextStyle {
      text: "".to_owned(),
      color: Color::RGB(0, 0, 0),
      line_height: 20,
      align: TextAlign::Left,
      pos: Point::new(0, 0),
    }
  }

  pub fn text(mut self, new_text: &str) -> Self {
    if new_text == "" {
      self.text = String::from(" ");
    } else {
      self.text = new_text.to_owned();
    }

    self
  }

  pub fn color(mut self, new_color: Color) -> Self {
    self.color = new_color;
    self
  }

  pub fn line_height(mut self, new_line_height: u32) -> Self {
    self.line_height = new_line_height;
    self
  }

  pub fn align(mut self, new_align: TextAlign) -> Self {
    self.align = new_align;
    self
  }

  pub fn pos(mut self, new_pos: Point) -> Self {
    self.pos = new_pos;
    self
  }

  pub fn to_rect(&self, aspect: f64) -> Rect {
    let (w, h) =
      ((aspect * self.line_height as f64) as u32, self.line_height);
    use TextAlign::*;

    match self.align {
      Left => Rect::new(self.pos.x(), self.pos.y(), w, h),
      Center => Rect::from_center(self.pos, w, h),
      Right => Rect::new(self.pos.x() - w as i32, self.pos.y(), w, h),
    }
  }
}
