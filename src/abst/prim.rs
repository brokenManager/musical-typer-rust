#[derive(Debug)]
pub struct Pos {
  x: f64,
  y: f64,
}

impl Pos {
  pub fn new<T: Into<f64>>(x: T, y: T) -> Self {
    Pos {
      x: x.into(),
      y: y.into(),
    }
  }

  pub fn x(&self) -> f64 {
    self.x
  }
  pub fn y(&self) -> f64 {
    self.y
  }
}

#[derive(Debug)]
pub struct Size {
  w: f64,
  h: f64,
}

impl Size {
  pub fn new<T: Into<f64>>(w: T, h: T) -> Self {
    Size {
      w: w.into(),
      h: h.into(),
    }
  }

  pub fn w(&self) -> f64 {
    self.w
  }
  pub fn h(&self) -> f64 {
    self.h
  }
}

#[derive(Debug)]
pub struct Area {
  pos: Pos,
  size: Size,
}

impl Area {
  pub fn new<T: Into<f64>>(x: T, y: T, w: T, h: T) -> Self {
    Area {
      pos: Pos::new(x, y),
      size: Size::new(w, h),
    }
  }

  pub fn x(&self) -> f64 {
    self.pos.x
  }
  pub fn y(&self) -> f64 {
    self.pos.y
  }
  pub fn w(&self) -> f64 {
    self.size.w
  }
  pub fn h(&self) -> f64 {
    self.size.h
  }
}

#[derive(Debug)]
pub struct Color {
  r: f64,
  g: f64,
  b: f64,
  a: f64,
}

impl Color {
  pub fn rgb<T: Into<f64>>(r: T, g: T, b: T) -> Self {
    Color {
      r: r.into(),
      g: g.into(),
      b: b.into(),
      a: 1.0,
    }
  }
  pub fn rgba<T: Into<f64>>(r: T, g: T, b: T, a: T) -> Self {
    Color {
      r: r.into(),
      g: g.into(),
      b: b.into(),
      a: a.into(),
    }
  }

  pub fn to_sdl(&self) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGBA(
      (self.r * u8::max_value() as f64) as u8,
      (self.g * u8::max_value() as f64) as u8,
      (self.b * u8::max_value() as f64) as u8,
      (self.a * u8::max_value() as f64) as u8,
    )
  }
}
