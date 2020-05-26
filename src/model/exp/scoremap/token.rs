use crate::model::exp::{
  scoremap::sections::section::note::sentence::roman::roman_str::RomanStr,
  time::MinuteSecond,
};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenContent {
  Property { key: String, value: String },
  Comment,
  Command(String),
  Lyrics(String),
  Yomigana(RomanStr),
  Caption(String),
  Section(String),
  Time(MinuteSecond),
}

#[derive(Debug, Clone)]
pub struct Token {
  pub line_num: usize,
  pub content: TokenContent,
}
