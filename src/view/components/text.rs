use crate::view::{
  renderer::{Component, RenderCtx, ViewResult},
  ViewError,
};
use sdl2::{render::Texture, surface::Surface, ttf::Font};
use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

mod style;
pub use style::*;

static TEXT_CACHE: Arc<
  RwLock<HashMap<TextStyle, (Texture<'static>, f64)>>,
> = Arc::new(RwLock::new(HashMap::new()));

#[derive(Debug)]
pub enum TextError {
  FontError(sdl2::ttf::FontError),
  TextureError(sdl2::render::TextureValueError),
  RenderError(String),
}

pub struct Text<'text, C> {
  style: TextStyle,
  font: &'text Font<'text, 'text>,
  creator: &'text C,
}

impl<'text, C> Text<'text, C>
where
  C: FnOnce(Surface) -> Result<Texture, TextError> + 'text,
{
  pub fn new(
    style: TextStyle,
    font: &'text Font,
    creator: &'text C,
  ) -> Result<Self, TextError> {
    Ok(Self {
      style,
      font,
      creator,
    })
  }
}

impl<'text, C> Component for Text<'text, C>
where
  C: FnOnce(Surface) -> Result<Texture, TextError> + 'text,
{
  type Props = TextStyle;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.style != new_props
  }

  fn update(&mut self, new_props: Self::Props) {
    self.style = new_props;
  }

  fn render(&self, ctx: RenderCtx<'_, '_>) -> ViewResult {
    if !TEXT_CACHE
      .read()
      .expect("reading TEXT_CACHE frozen")
      .contains_key(&self.style)
    {
      let &Self {
        font,
        style,
        creator,
      } = &self;
      let &TextStyle { text, color, .. } = &self.style;
      let aspect = {
        let (w, h) =
          font.size_of(&text).map_err(|e| ViewError::FontError {
            message: e.to_string(),
          })?;
        w as f64 / h as f64
      };
      let text = if text == "" { " " } else { &text };
      let surface = font
        .render(text)
        .blended(color.clone())
        .map_err(|e| ViewError::FontError {
          message: e.to_string(),
        })?;

      let texture =
        creator(surface).map_err(|e| ViewError::TextError(e))?;

      TEXT_CACHE
        .write()
        .expect("writing TEXT_CACHE frozen")
        .insert(style.clone(), (texture, aspect));
    }
    let (texture, aspect) = TEXT_CACHE
      .read()
      .expect("reading TEXT_CACHE frozen")
      .get(&self.style)
      .expect("missed caching texture");

    ctx
      .borrow_mut()
      .paste_texture(&texture, self.style.to_rect(*aspect))
  }
}
