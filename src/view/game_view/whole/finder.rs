use sdl2::{
  pixels::Color,
  rect::{Point, Rect},
};

use crate::{
  model::exp::sentence::{Sentence, TypingStr},
  view::renderer::{
    text::TextAlign, Component, Renderer, ViewResult,
  },
};

#[derive(PartialEq)]
pub struct FinderProps<'a> {
  pub sentence: &'a Sentence,
  pub remaining_ratio: f64,
}

pub struct Finder<'a> {
  props: FinderProps<'a>,
  client: Rect,
}

impl<'a> Finder<'a> {
  pub fn new(
    mut initial_props: FinderProps<'a>,
    client: Rect,
  ) -> Self {
    initial_props.remaining_ratio =
      initial_props.remaining_ratio.max(0.).min(1.);
    Self {
      props: initial_props,
      client,
    }
  }
}

impl<'a> Component for Finder<'a> {
  type Props = FinderProps<'a>;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.props != new_props
  }

  fn update(&mut self, new_props: Self::Props) {
    self.props = new_props;
  }

  fn render(&self, canvas: &mut Renderer<'_, '_>) -> ViewResult {
    let &Finder { props, client } = &self;
    let &FinderProps {
      remaining_ratio,
      sentence,
    } = &props;

    canvas.set_draw_color(Color::RGB(230, 220, 200));
    canvas.fill_rect(client.clone())?;

    let remaining_width =
      (client.width() as f64 * remaining_ratio) as u32;
    canvas.set_draw_color(Color::RGB(203, 193, 176));
    canvas.fill_rect(Rect::new(
      client.x(),
      client.y(),
      remaining_width,
      client.height(),
    ))?;

    const JAPANESE_HEIGHT: u32 = 30;
    let half_x = client.width() / 2;
    let will_input_japanese = sentence.origin();
    canvas.text(|s| {
      s.color(Color::RGB(80, 80, 80))
        .text(will_input_japanese)
        .line_height(JAPANESE_HEIGHT)
        .align(TextAlign::Left)
        .pos(client.top_left())
    })?;

    const ROMAN_HEIGHT: u32 = 40;
    {
      let TypingStr {
        will_input,
        inputted,
      } = sentence.roman();
      let will_input = will_input.as_str();
      let inputted = inputted.as_str();

      canvas.text(|s| {
        s.color(Color::RGB(0, 0, 0))
          .text(will_input)
          .line_height(ROMAN_HEIGHT)
          .align(TextAlign::Left)
          .pos(Point::new(
            half_x as i32 + 5,
            client.bottom() - ROMAN_HEIGHT as i32 - 20,
          ))
      })?;

      canvas.text(|s| {
        s.color(Color::RGB(80, 80, 80))
          .text(inputted)
          .line_height(ROMAN_HEIGHT)
          .align(TextAlign::Right)
          .pos(Point::new(
            half_x as i32 - 5,
            client.bottom() - ROMAN_HEIGHT as i32 - 20,
          ))
      })?;
    }
    const YOMIGANA_HEIGHT: u32 = 80;
    {
      let TypingStr {
        will_input,
        inputted,
      } = sentence.yomiagana();
      let will_input = will_input.as_str();
      let inputted = inputted.as_str();

      canvas.text(|s| {
        s.color(Color::RGB(0, 0, 0))
          .text(will_input)
          .line_height(YOMIGANA_HEIGHT)
          .align(TextAlign::Left)
          .pos(Point::new(
            half_x as i32 + 5,
            client.bottom()
              - ROMAN_HEIGHT as i32
              - YOMIGANA_HEIGHT as i32
              - 20,
          ))
      })?;

      canvas.text(|s| {
        s.color(Color::RGB(80, 80, 80))
          .text(inputted)
          .line_height(YOMIGANA_HEIGHT)
          .align(TextAlign::Right)
          .pos(Point::new(
            half_x as i32 - 5,
            client.bottom()
              - ROMAN_HEIGHT as i32
              - YOMIGANA_HEIGHT as i32
              - 20,
          ))
      })?;
    }

    Ok(())
  }
}
