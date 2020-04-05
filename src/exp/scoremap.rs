use std::collections::HashMap;

use super::note::Note;

pub mod lexer;
pub mod parser;

use lexer::ScoremapLexError;
use parser::ScoremapParseError;

#[derive(Debug)]
pub enum ScoremapError {
  LexError(ScoremapLexError),
  ParseError(ScoremapParseError),
}

pub type ScoremapMetadata = HashMap<String, String>;

#[derive(Debug)]
pub struct Scoremap {
  pub metadata: ScoremapMetadata,
  pub notes: Vec<Note>,
}

impl Scoremap {
  pub fn from_file<C>(
    file: std::fs::File,
    configurator: C,
  ) -> Result<Self, ScoremapError>
  where
    C: FnOnce(lexer::ScoremapLoadConfig) -> lexer::ScoremapLoadConfig,
  {
    use ScoremapError::*;

    use std::io::BufReader;
    let reader = BufReader::new(file);
    let tokens = lexer::lex(
      configurator(lexer::ScoremapLoadConfig::new()),
      reader,
    )
    .map_err(|e| LexError(e))?;

    Ok(parser::parse(tokens.into_iter()).map_err(|e| ParseError(e))?)
  }
}
