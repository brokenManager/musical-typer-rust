use super::{
  super::super::section::note::sentence::roman::roman_str::RomanStr,
  lex, ScoremapLexError, ScoremapLoadConfig,
};
use crate::model::exp::{
  scoremap::token::{Token, TokenContent},
  time::MinuteSecond,
};
use std::io::BufReader;

#[test]
fn case2() -> Result<(), ScoremapLexError> {
  let expected_notes: Vec<Token> = vec![
    Token {
      line_num: 1,
      content: TokenContent::Comment,
    },
    Token {
      line_num: 2,
      content: TokenContent::Comment,
    },
    Token {
      line_num: 3,
      content: TokenContent::Property {
        key: "title".into(),
        value: "TEST".into(),
      },
    },
    Token {
      line_num: 4,
      content: TokenContent::Property {
        key: "score_author".into(),
        value: "Mikuro さいな".into(),
      },
    },
    Token {
      line_num: 5,
      content: TokenContent::Property {
        key: "song_data".into(),
        value: "void.ogg".into(),
      },
    },
    Token {
      line_num: 6,
      content: TokenContent::Property {
        key: "bpm".into(),
        value: "222.22".into(),
      },
    },
    Token {
      line_num: 7,
      content: TokenContent::Comment,
    },
    Token {
      line_num: 8,
      content: TokenContent::Command("start".into()),
    },
    Token {
      line_num: 9,
      content: TokenContent::Time(MinuteSecond::new().seconds(2.22)),
    },
    Token {
      line_num: 10,
      content: TokenContent::Lyrics("打鍵テスト".into()),
    },
    Token {
      line_num: 11,
      content: TokenContent::Yomigana(
        RomanStr::new("だけんてすと").unwrap(),
      ),
    },
    Token {
      line_num: 12,
      content: TokenContent::Comment,
    },
    Token {
      line_num: 13,
      content: TokenContent::Time(MinuteSecond::new().seconds(3.0)),
    },
    Token {
      line_num: 14,
      content: TokenContent::Command("end".into()),
    },
  ];

  let reader = BufReader::new(
    r#"
# Sample 1
:title TEST
:score_author Mikuro さいな
:song_data void.ogg
:bpm 222.22

[start]
*2.22
打鍵テスト
:だけんてすと

*3.0
[end]
"#
    .as_bytes(),
  );
  let tokens = lex(
    ScoremapLoadConfig::new().ignore_invalid_properties(true),
    reader,
  )?;

  for (index, (expected, actual)) in
    expected_notes.iter().zip(tokens.iter()).enumerate()
  {
    assert_eq!(
      expected.content, actual.content,
      "content | token_index: {}",
      index
    );
    assert_eq!(
      expected.line_num, actual.line_num,
      "line_num | token_index: {}",
      index
    );
  }
  assert_eq!(expected_notes.len(), tokens.len());

  Ok(())
}

#[test]
fn pattern_tests() -> Result<(), super::ScoremapLexError> {
  use super::pattern::*;
  use super::ScoremapLexError;
  use regex::Regex;

  fn captures_vec<'a>(
    this: &'a Regex,
    text: &'a str,
  ) -> Result<Vec<&'a str>, ScoremapLexError> {
    Ok(
      this
        .captures(text)
        .ok_or(ScoremapLexError::CaptureFailure)?
        .iter()
        .skip(1)
        .map(|m: Option<regex::Match>| m.map_or("", |m| m.as_str()))
        .collect::<Vec<_>>(),
    )
  }

  let reg = Regex::new(PROPERTY)?;
  assert_eq!(
    captures_vec(&reg, ":title        キミのチカラ")?,
    vec!["title", "キミのチカラ"]
  );
  assert_eq!(
    captures_vec(&reg, ":song_author  佐々木英州")?,
    vec!["song_author", "佐々木英州"]
  );
  assert_eq!(
    captures_vec(&reg, ":singer       初音ミク")?,
    vec!["singer", "初音ミク"]
  );
  assert_eq!(
    captures_vec(&reg, ":score_author Colk")?,
    vec!["score_author", "Colk"]
  );
  assert_eq!(
    captures_vec(&reg, ":song_data    kkiminochikara-edited.wav")?,
    vec!["song_data", "kkiminochikara-edited.wav"]
  );

  let reg = Regex::new(COMMENT)?;
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

  let reg = Regex::new(COMMAND)?;
  assert!(reg.is_match("[start]"));
  assert!(reg.is_match(" [ end ] "));
  assert!(reg.is_match("[break] "));
  assert!(reg.is_match(" [start ]"));
  assert!(reg.is_match(" [end ] "));
  assert!(reg.is_match("[ break]"));
  assert!(reg.is_match("[ start] "));

  let reg = Regex::new(YOMIGANA)?;
  assert!(reg.is_match(":てすと"));
  assert!(reg.is_match(":はんばーがー"));
  assert!(reg.is_match(":ぅゎょぅじょっょぃ"));

  let reg = Regex::new(CAPTION)?;
  assert!(reg.is_match(">>テスト"));
  assert!(reg.is_match(">>HAMBURGER"));

  let reg = Regex::new(SECONDS)?;
  assert!(reg.is_match("*2.0"));
  assert!(reg.is_match("* 1.423523"));
  assert!(reg.is_match("*0.020"));
  assert!(reg.is_match("* 1223.20"));
  assert!(reg.is_match("*01.2"));

  assert!(!reg.is_match("* 03."));
  assert!(!reg.is_match("*7."));
  assert!(!reg.is_match("*.5"));

  let reg = Regex::new(MINUTES)?;
  assert!(reg.is_match("|3"));
  assert!(reg.is_match("| 4"));

  let reg = Regex::new(SECTION)?;
  assert!(reg.is_match("@Aメロ"));
  assert!(reg.is_match("@ †ラップ† "));

  Ok(())
}
