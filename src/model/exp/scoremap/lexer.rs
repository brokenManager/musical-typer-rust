use regex::Regex;
use std::io::{BufReader, Read};

use super::token::Token;
use pattern::{LexerCtx, TokenResult, Tokenizer};

mod pattern;
#[cfg(test)]
mod tests;

impl From<regex::Error> for ScoremapLexError {
  fn from(err: regex::Error) -> ScoremapLexError {
    ScoremapLexError::InternalRegexCompileFailure(err)
  }
}

#[derive(Debug)]
pub enum ScoremapLexError {
  InternalRegexCompileFailure(regex::Error),
  CaptureFailure,
  ParsingNumberFailure,
  UnexpectedEndOfFile,
  InvalidPropertyDefinition {
    line_num: usize,
    reason: &'static str,
  },
  InvalidStatementDefinition {
    line_num: usize,
    reason: &'static str,
  },
  UnknownToken,
}

#[derive(Debug, Copy, Clone)]
pub struct ScoremapLoadConfig {
  ignore_unsupported_property: bool,
}

impl ScoremapLoadConfig {
  pub fn new() -> Self {
    ScoremapLoadConfig {
      ignore_unsupported_property: false,
    }
  }

  pub fn ignore_unsupported_property(
    mut self,
    whether: bool,
  ) -> Self {
    self.ignore_unsupported_property = whether;
    self
  }
}

struct Lexer {
  pattern: Regex,
  func: Tokenizer,
  next: Option<Box<Lexer>>,
}

impl Lexer {
  fn new(pattern: Regex, f: Tokenizer) -> Self {
    Self {
      pattern,
      func: f,
      next: None,
    }
  }

  fn connect(self, other: Self) -> Self {
    let next = if let Some(next) = self.next {
      next.connect(other)
    } else {
      other
    };
    Self {
      next: Some(Box::new(next)),
      ..self
    }
  }

  fn lex(&mut self, ctx: &mut LexerCtx) -> TokenResult {
    let line = ctx.line();
    self
      .pattern
      .captures(&line)
      .and_then(|captures| (self.func)(captures, ctx))
      .or_else(|| self.next.as_mut().and_then(|next| next.lex(ctx)))
  }
}

pub fn lex<T>(
  config: ScoremapLoadConfig,
  reader: BufReader<T>,
) -> Result<Vec<Token>, ScoremapLexError>
where
  T: Read,
{
  use pattern::*;
  use std::io::BufRead;
  use ScoremapLexError::*;

  let comment_reg = Regex::new(COMMENT)?;
  let comment = Lexer::new(comment_reg, comment_lexer);

  let property_reg = Regex::new(PROPERTY)?;
  let property = Lexer::new(property_reg, property_lexer);

  let command_reg = Regex::new(COMMAND)?;
  let command = Lexer::new(command_reg, command_lexer);

  let yomigana_reg = Regex::new(YOMIGANA)?;
  let yomigana = Lexer::new(yomigana_reg, yomigana_lexer);

  let caption_reg = Regex::new(CAPTION)?;
  let caption = Lexer::new(caption_reg, caption_lexer);

  let section_reg = Regex::new(SECTION)?;
  let section = Lexer::new(section_reg, section_lexer);

  let seconds_reg = Regex::new(SECONDS)?;
  let seconds = Lexer::new(seconds_reg, seconds_lexer);

  let minutes_reg = Regex::new(MINUTES)?;
  let minutes = Lexer::new(minutes_reg, minutes_lexer);

  let lyrics = Lexer::new(Regex::new(r".+")?, lyrics_lexer);

  let mut entire = comment
    .connect(seconds)
    .connect(minutes)
    .connect(command)
    .connect(caption)
    .connect(property)
    .connect(yomigana)
    .connect(section)
    .connect(lyrics);

  let mut ctx = LexerCtx::new(config);
  let mut tokens: Vec<Token> = vec![];
  for (line_num, line) in reader.lines().enumerate() {
    let line_num = line_num + 1; // starts from 1
    let line = line.map_err(|_e| UnexpectedEndOfFile)?;
    ctx.set_line(line, line_num);
    if let Some(token) = entire.lex(&mut ctx).transpose()? {
      tokens.push(token);
    }
  }
  Ok(tokens)
}
