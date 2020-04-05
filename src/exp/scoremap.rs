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
  pub fn from_file(
    file: std::fs::File,
    config: lexer::ScoremapLoadConfig,
  ) -> Result<Self, ScoremapError> {
    use ScoremapError::*;

    use std::io::BufReader;
    let reader = BufReader::new(file);
    let tokens =
      lexer::lex(config, reader).map_err(|e| LexError(e))?;

    Ok(parser::parse(tokens.into_iter()).map_err(|e| ParseError(e))?)
  }
}
