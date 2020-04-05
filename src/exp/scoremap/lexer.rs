use regex::Regex;
use std::io::BufReader;

use crate::exp::roman::roman_str::RomanStr;

const PROPERTY: &str = r"^:([[:^space:]]+)[[:space:]]+(.+)$";
const COMMENT: &str = r"^[[:space:]]*(:?#.*)?$";
const COMMAND: &str =
  r"^[[:space:]]*\[[[:space:]]*(.*)[[:space:]]*\][[:space:]]*$";
const YOMIGANA: &str = r"^:([あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわゐゑをんぁぃぅぇぉゃゅょゎっーがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽ]+)$";
const CAPTION: &str = r"^[[:space:]]*>>[[:space:]]*(.+)[[:space:]]*$";
const SECTION: &str = r"@[[:space:]]*[[:space:]]*(.+)[[:space:]]*$";
const SECONDS: &str =
  r"^\*[[:space:]]*((?:[0-9]+\.[0-9]+)|(?:0\.[0-9]+))[[:space:]]*$";
const MINUTES: &str = r"^\|[[:space:]]*([1-9][0-9]*)[[:space:]]*$";

fn captures_vec<'a>(this: &'a Regex, text: &'a str) -> Vec<&'a str> {
  this
    .captures(text)
    .unwrap()
    .iter()
    .skip(1)
    .map(|capture| capture.unwrap().as_str())
    .collect::<Vec<_>>()
}

#[test]
fn pattern_tests() {
  let reg = Regex::new(PROPERTY).unwrap();
  assert_eq!(
    captures_vec(&reg, ":title        キミのチカラ"),
    vec!["title", "キミのチカラ"]
  );
  assert_eq!(
    captures_vec(&reg, ":song_author  佐々木英州"),
    vec!["song_author", "佐々木英州"]
  );
  assert_eq!(
    captures_vec(&reg, ":singer       初音ミク"),
    vec!["singer", "初音ミク"]
  );
  assert_eq!(
    captures_vec(&reg, ":score_author Colk"),
    vec!["score_author", "Colk"]
  );
  assert_eq!(
    captures_vec(&reg, ":song_data    kkiminochikara-edited.wav"),
    vec!["song_data", "kkiminochikara-edited.wav"]
  );

  let reg = Regex::new(COMMENT).unwrap();
  assert!(reg.is_match("# This is a comment. "));
  assert!(reg.is_match("  # Indented!"));
  assert!(reg.is_match(""));

  assert!(!reg.is_match("[break] "));
  assert!(!reg.is_match(" [start ]"));
  assert!(!reg.is_match(":はんばーがー"));
  assert!(!reg.is_match(">>テスト"));
  assert!(!reg.is_match("*2.0"));
  assert!(!reg.is_match("* 1.423523"));
  assert!(!reg.is_match("* 03."));
  assert!(!reg.is_match("*7."));
  assert!(!reg.is_match("|3"));

  let reg = Regex::new(COMMAND).unwrap();
  assert!(reg.is_match("[start]"));
  assert!(reg.is_match(" [ end ] "));
  assert!(reg.is_match("[break] "));
  assert!(reg.is_match(" [start ]"));
  assert!(reg.is_match(" [end ] "));
  assert!(reg.is_match("[ break]"));
  assert!(reg.is_match("[ start] "));

  let reg = Regex::new(YOMIGANA).unwrap();
  assert!(reg.is_match(":てすと"));
  assert!(reg.is_match(":はんばーがー"));
  assert!(reg.is_match(":ぅゎょぅじょっょぃ"));

  let reg = Regex::new(CAPTION).unwrap();
  assert!(reg.is_match(">>テスト"));
  assert!(reg.is_match(">>HAMBURGER"));

  let reg = Regex::new(SECONDS).unwrap();
  assert!(reg.is_match("*2.0"));
  assert!(reg.is_match("* 1.423523"));
  assert!(reg.is_match("*0.020"));
  assert!(reg.is_match("* 1223.20"));
  assert!(reg.is_match("*01.2"));

  assert!(!reg.is_match("* 03."));
  assert!(!reg.is_match("*7."));
  assert!(!reg.is_match("*.5"));

  let reg = Regex::new(MINUTES).unwrap();
  assert!(reg.is_match("|3"));
  assert!(reg.is_match("| 4"));

  let reg = Regex::new(SECTION).unwrap();
  assert!(reg.is_match("@Aメロ"));
  assert!(reg.is_match("@ †ラップ† "));
}

const METADATA_KEYS: &[&'static str] = &[
  "title",
  "song_author",
  "singer",
  "score_author",
  "song_data",
  "bpm",
];

#[derive(Debug)]
pub enum ScoremapLexError {
  UnexceptedEndOfFile,
  InvalidPropertyDeifinition {
    line_num: usize,
    reason: &'static str,
  },
  InvalidStatementDefinition {
    line_num: usize,
    reason: &'static str,
  },
}

#[derive(Debug)]
pub enum TokenContent {
  Property { key: String, value: String },
  Command(String),
  Lyrics(String),
  Yomigana(RomanStr),
  Caption(String),
  Section(String),
  Seconds(f64),
  Minutes(u32),
}

#[derive(Debug)]
pub struct Token {
  pub line_num: usize,
  pub content: TokenContent,
}

#[derive(Debug, Copy, Clone)]
pub struct ScoremapLoadConfig {
  ignore_invalid_properties: bool,
}

impl ScoremapLoadConfig {
  pub fn new() -> Self {
    ScoremapLoadConfig {
      ignore_invalid_properties: false,
    }
  }

  pub fn ignore_invalid_properties(mut self, whether: bool) -> Self {
    self.ignore_invalid_properties = whether;
    self
  }
}

pub fn lex<T>(
  config: ScoremapLoadConfig,
  reader: BufReader<T>,
) -> Result<Vec<Token>, ScoremapLexError>
where
  T: std::io::Read,
{
  use std::io::BufRead;
  use ScoremapLexError::*;
  use TokenContent::*;

  let property_reg = Regex::new(PROPERTY).unwrap();
  let comment_reg = Regex::new(COMMENT).unwrap();
  let command_reg = Regex::new(COMMAND).unwrap();
  let yomigana_reg = Regex::new(YOMIGANA).unwrap();
  let caption_reg = Regex::new(CAPTION).unwrap();
  let section_reg = Regex::new(SECTION).unwrap();
  let seconds_reg = Regex::new(SECONDS).unwrap();
  let minutes_reg = Regex::new(MINUTES).unwrap();

  let mut tokens: Vec<Token> = vec![];

  for (line_num, line) in reader.lines().enumerate() {
    let line_num = line_num + 1; // starts from 1
    let line = &line.map_err(|_e| UnexceptedEndOfFile)?;
    if comment_reg.is_match(line) {
      continue;
    }
    if let Some(seconds) = seconds_reg.captures(line) {
      let num: f64 =
        seconds.get(1).unwrap().as_str().parse().unwrap();
      tokens.push(Token {
        line_num,
        content: Seconds(num),
      });
      continue;
    }
    if let Some(minutes) = minutes_reg.captures(line) {
      let num: u32 =
        minutes.get(1).unwrap().as_str().parse().unwrap();
      tokens.push(Token {
        line_num,
        content: Minutes(num),
      });
      continue;
    }
    if let Some(command) = command_reg.captures(line) {
      let string = command.get(1).unwrap().as_str();
      tokens.push(Token {
        line_num,
        content: Command(string.to_owned()),
      });
      continue;
    }
    if let Some(caption) = caption_reg.captures(line) {
      let string = caption.get(1).unwrap().as_str();
      tokens.push(Token {
        line_num,
        content: Caption(string.to_owned()),
      });
      continue;
    }
    if let Some(property) = property_reg.captures(line) {
      if property.len() != 3 {
        return Err(InvalidPropertyDeifinition {
          line_num,
          reason: "プロパティの指定が正しくありません。",
        });
      }
      let key = property.get(1).unwrap().as_str().to_owned();
      if !METADATA_KEYS.contains(&key.as_str()) {
        if config.ignore_invalid_properties {
          println!("未対応のプロパティがありました。無視します。");
          continue;
        }
        return Err(InvalidPropertyDeifinition {
          line_num,
          reason: "未対応のプロパティです。",
        });
      }
      let value = property.get(2).unwrap().as_str().to_owned();
      tokens.push(Token {
        line_num,
        content: Property { key, value },
      });
      continue;
    }
    if let Some(yomigana) = yomigana_reg.captures(line) {
      let string = yomigana.get(1).unwrap().as_str();
      tokens.push(Token {
        line_num,
        content: Yomigana(RomanStr::new(string).map_err(|_e| {
          InvalidStatementDefinition {
            line_num,
            reason:
              "ふりがなでのそのような平仮名の並びは未対応です。",
          }
        })?),
      });
    }
    if let Some(section) = section_reg.captures(line) {
      let string = section.get(1).unwrap().as_str();
      tokens.push(Token {
        line_num,
        content: Section(string.to_owned()),
      });
      continue;
    }
    // どのパターンにも一致しない場合は文指定なので
    tokens.push(Token {
      line_num,
      content: Lyrics(line.to_owned()),
    });
  }
  Ok(tokens)
}
