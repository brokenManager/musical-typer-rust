use sections::Sections;
use std::{collections::HashMap, fs::File};

pub mod lexer;
pub mod parser;
pub mod sections;
mod tests;
pub mod token;

use lexer::{ScoremapLexError, ScoremapLoadConfig};
use parser::ScoremapParseError;

#[derive(Debug)]
pub enum ScoremapError {
  LexError(ScoremapLexError),
  ParseError(ScoremapParseError),
}

#[readonly::make]
#[derive(PartialEq, Clone)]
pub struct MusicInfo {
  pub title: String,
  pub song_author: String,
}

#[derive(Debug, Clone)]
pub struct ScoremapMetadata(HashMap<String, String>);

impl ScoremapMetadata {
  pub fn new() -> Self {
    Self(HashMap::new())
  }

  pub fn get(&self, key: &str) -> Option<&String> {
    self.0.get(key)
  }

  pub fn get_music_info(&self) -> MusicInfo {
    MusicInfo {
      title: self
        .0
        .get("title")
        .cloned()
        .unwrap_or("曲名不詳".into()),
      song_author: self
        .0
        .get("song_author")
        .cloned()
        .unwrap_or("作曲者不詳".into()),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Scoremap {
  pub metadata: ScoremapMetadata,
  pub sections: Sections,
}

impl Scoremap {
  pub fn from_str<C>(
    string: &str,
    configurator: C,
  ) -> Result<Self, ScoremapError>
  where
    C: FnOnce(ScoremapLoadConfig) -> ScoremapLoadConfig,
  {
    use ScoremapError::*;

    use std::io::BufReader;
    let reader = BufReader::new(string.as_bytes());
    let tokens = lexer::lex(
      configurator(lexer::ScoremapLoadConfig::new()),
      reader,
    )
    .map_err(|e| LexError(e))?;
    Ok(parser::parse(&tokens).map_err(|e| ParseError(e))?)
  }

  pub fn from_file<C>(
    file: File,
    configurator: C,
  ) -> Result<Self, ScoremapError>
  where
    C: FnOnce(ScoremapLoadConfig) -> ScoremapLoadConfig,
  {
    use ScoremapError::*;

    use std::io::BufReader;
    let reader = BufReader::new(file);
    let tokens = lexer::lex(
      configurator(lexer::ScoremapLoadConfig::new()),
      reader,
    )
    .map_err(|e| LexError(e))?;

    Ok(parser::parse(&tokens).map_err(|e| ParseError(e))?)
  }
}
