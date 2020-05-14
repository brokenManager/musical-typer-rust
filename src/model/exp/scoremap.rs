use section::Sections;
use std::{collections::HashMap, fs::File};

pub mod lexer;
pub mod parser;
pub mod section;
mod tests;
pub mod token;

use lexer::{ScoremapLexError, ScoremapLoadConfig};
use parser::ScoremapParseError;

#[derive(Debug)]
pub enum ScoremapError {
  LexError(ScoremapLexError),
  ParseError(ScoremapParseError),
}

pub type ScoremapMetadata = HashMap<String, String>;

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
