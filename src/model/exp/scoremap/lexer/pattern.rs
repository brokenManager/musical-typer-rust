use super::{
  super::token::TokenContent::*, ScoremapLexError, ScoremapLoadConfig,
};
use crate::model::exp::{
  scoremap::token::Token, sentence::roman::RomanStr,
  time::MinuteSecond,
};
use regex::Captures;
use ScoremapLexError::*;

const METADATA_KEYS: &[&str] = &[
  "title",
  "song_author",
  "singer",
  "score_author",
  "song_data",
  "bpm",
];

#[derive(Debug, Clone)]
pub struct LexerCtx {
  line_num: usize,
  line: String,
  cfg: ScoremapLoadConfig,
  curr_mise: MinuteSecond,
}

impl LexerCtx {
  pub fn new(config: ScoremapLoadConfig) -> Self {
    Self {
      cfg: config,
      curr_mise: MinuteSecond::new(),
      line_num: 0,
      line: "".into(),
    }
  }
  pub fn line(&self) -> String {
    self.line.clone()
  }
  pub fn set_line(&mut self, line: String, line_num: usize) {
    self.line = line;
    self.line_num = line_num;
  }
}

pub type TokenResult = Option<Result<Token, ScoremapLexError>>;
pub type Tokenizer = fn(Captures, &mut LexerCtx) -> TokenResult;

pub const COMMENT: &str = r"^[[:space:]]*(:?#.*)?$";
pub fn comment_lexer(
  _: Captures,
  LexerCtx { line_num, .. }: &mut LexerCtx,
) -> TokenResult {
  Some(Ok(Token {
    line_num: *line_num,
    content: Comment,
  }))
}

pub const PROPERTY: &str = r"^:([[:^space:]]+)[[:space:]]+(.+)$";
pub fn property_lexer(
  captures: Captures,
  LexerCtx { line_num, cfg, .. }: &mut LexerCtx,
) -> TokenResult {
  let line_num = *line_num;
  if captures.len() != 3 {
    return Some(Err(InvalidPropertyDefinition {
      line_num,
      reason: "プロパティの指定が正しくありません。",
    }));
  }
  let key = captures.get(1)?.as_str().to_owned();
  if !METADATA_KEYS.contains(&key.as_str()) {
    return Some(if cfg.ignore_unsupported_property {
      println!("未対応のプロパティがありました。無視します。");
      Ok(Token {
        line_num,
        content: Comment,
      })
    } else {
      Err(InvalidPropertyDefinition {
        line_num,
        reason: "未対応のプロパティです。",
      })
    });
  }
  let value = captures.get(2)?.as_str().to_owned();
  Some(Ok(Token {
    line_num,
    content: Property { key, value },
  }))
}

pub const COMMAND: &str =
  r"^[[:space:]]*\[[[:space:]]*(.*)[[:space:]]*\][[:space:]]*$";
pub fn command_lexer(
  captures: Captures,
  LexerCtx { line_num, .. }: &mut LexerCtx,
) -> TokenResult {
  let string = captures.get(1)?.as_str();
  Some(Ok(Token {
    line_num: *line_num,
    content: Command(string.to_owned()),
  }))
}

pub const YOMIGANA: &str = r"^:([あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわゐゑをんぁぃぅぇぉゃゅょゎっーがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽ]+)$";
pub fn yomigana_lexer(
  captures: Captures,
  LexerCtx { line_num, .. }: &mut LexerCtx,
) -> TokenResult {
  let line_num = *line_num;
  let string = captures.get(1)?.as_str();
  let content = Yomigana({
    let roman = RomanStr::new(string).map_err(|_e| {
      InvalidStatementDefinition {
        line_num,
        reason: "ふりがなでのそのような平仮名の並びは未対応です。",
      }
    });
    if let Err(err) = roman {
      return Some(Err(err));
    }
    roman.unwrap()
  });
  Some(Ok(Token { line_num, content }))
}

pub const CAPTION: &str =
  r"^[[:space:]]*>>[[:space:]]*(.+)[[:space:]]*$";
pub fn caption_lexer(
  captures: Captures,
  LexerCtx { line_num, .. }: &mut LexerCtx,
) -> TokenResult {
  let string = captures.get(1)?.as_str();
  Some(Ok(Token {
    line_num: *line_num,
    content: Caption(string.to_owned()),
  }))
}

pub const SECTION: &str =
  r"@[[:space:]]*[[:space:]]*(.+)[[:space:]]*$";
pub fn section_lexer(
  captures: Captures,
  LexerCtx { line_num, .. }: &mut LexerCtx,
) -> TokenResult {
  let string = captures.get(1)?.as_str();
  Some(Ok(Token {
    line_num: *line_num,
    content: Section(string.to_owned()),
  }))
}

pub const SECONDS: &str =
  r"^\*[[:space:]]*((?:[0-9]+\.[0-9]+)|(?:0\.[0-9]+))[[:space:]]*$";
pub fn seconds_lexer(
  captures: Captures,
  LexerCtx {
    line_num,
    curr_mise,
    ..
  }: &mut LexerCtx,
) -> TokenResult {
  let num: f64 = {
    let num_res = captures
      .get(1)?
      .as_str()
      .parse()
      .map_err(|_e| ParsingNumberFailure);
    if let Err(err) = num_res {
      return Some(Err(err));
    }
    num_res.unwrap()
  };
  // それ以前の時間指定は無視
  if curr_mise.seconds(num) <= *curr_mise {
    return None;
  }
  *curr_mise = curr_mise.seconds(num);
  Some(Ok(Token {
    line_num: *line_num,
    content: Time(curr_mise.clone()),
  }))
}

pub const MINUTES: &str =
  r"^\|[[:space:]]*([1-9][0-9]*)[[:space:]]*$";
pub fn minutes_lexer(
  captures: Captures,
  LexerCtx {
    curr_mise,
    line_num,
    ..
  }: &mut LexerCtx,
) -> TokenResult {
  let num: u32 = {
    let num_res = captures
      .get(1)?
      .as_str()
      .parse()
      .map_err(|_e| ParsingNumberFailure);
    if let Err(err) = num_res {
      return Some(Err(err));
    }
    num_res.unwrap()
  };
  *curr_mise = curr_mise.minutes(num).seconds(0.0);
  Some(Ok(Token {
    line_num: *line_num,
    content: Comment,
  }))
}

pub fn lyrics_lexer(
  _: Captures,
  LexerCtx { line_num, line, .. }: &mut LexerCtx,
) -> TokenResult {
  Some(Ok(Token {
    line_num: *line_num,
    content: Lyrics(line.to_owned()),
  }))
}
