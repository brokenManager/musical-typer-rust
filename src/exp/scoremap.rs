use std::collections::HashMap;

use regex::Regex;

use super::note::Note;

#[derive(Debug)]
pub enum ScoremapError {
  UnexceptedEndOfFile,
  InvalidCommand { line_num: u64, reason: &'static str },
  InvalidPropertyDeifinition { line_num: u64, reason: &'static str },
  InvalidStatementDeifinition { line_num: u64, reason: &'static str },
}

const METADATA_KEYS: &[&'static str] = &[
  "title",
  "song_author",
  "singer",
  "score_author",
  "song_data",
  "bpm",
];

pub type ScoremapMetadata = HashMap<String, String>;

pub struct Scoremap {
  metadata: ScoremapMetadata,
  notes: Vec<Note>,
}

const PROPERTY: &str =
  r"^:([[:^space:]]+)[[:space:]]+([[:^space:]]+)$";
const COMMENT: &str = r"^[[:space:]]*#?.*$";
const COMMAND: &str =
  r"^[[:space:]]*\[[[:space:]]*(.*)[[:space:]]*\][[:space:]]*$";
const HIRAGANA: &str = r"^:([あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわゐゑをんぁぃぅぇぉゃゅょゎっーがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽ]+)$";
const CAPTION: &str = r"^>>(.+)$";

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

  let reg = Regex::new(COMMAND).unwrap();
  assert!(reg.is_match("[start]"));
  assert!(reg.is_match(" [ end ] "));
  assert!(reg.is_match("[break] "));
  assert!(reg.is_match(" [start ]"));
  assert!(reg.is_match(" [end ] "));
  assert!(reg.is_match("[ break]"));
  assert!(reg.is_match("[ start] "));

  let reg = Regex::new(HIRAGANA).unwrap();
  assert!(reg.is_match(":てすと"));
  assert!(reg.is_match(":はんばーがー"));
  assert!(reg.is_match(":ぅゎょぅじょっょぃ"));

  let reg = Regex::new(CAPTION).unwrap();
  assert!(reg.is_match(">>テスト"));
  assert!(reg.is_match(">>HAMBURGER"));
}

impl Scoremap {
  pub fn from_file(file: std::fs::File) -> Result<(), ScoremapError> {
    use ScoremapError::*;

    let property_reg = Regex::new(PROPERTY).unwrap();
    let comment_reg = Regex::new(COMMENT).unwrap();
    let command_reg = Regex::new(COMMAND).unwrap();
    let hiragana_reg = Regex::new(HIRAGANA).unwrap();
    let caption_reg = Regex::new(CAPTION).unwrap();

    let mut metadata = ScoremapMetadata::new();
    let mut notes: Vec<Note> = vec![];
    let mut line_num = 1;
    let mut parsing_lyrics = false;
    let mut line_time = 0.0;

    use std::io::{BufRead, BufReader};
    let reader = BufReader::new(file);
    for line in reader.lines() {
      let line = &line.map_err(|_e| UnexceptedEndOfFile)?;
      if comment_reg.is_match(line) {
        continue;
      }
      if let Some(command) = command_reg.captures(line) {
        let string = command.get(0).unwrap().as_str();
        match string {
          "start" => {
            if parsing_lyrics {
              return Err(InvalidCommand {
                line_num,
                reason:
                  "start コマンドは end コマンドより前で有効です。",
              });
            }
            parsing_lyrics = true;
          }
          "break" => {}
          "end" => {
            if !parsing_lyrics {
              return Err(InvalidCommand {
                line_num,
                reason:
                  "end コマンドは start コマンドより後で有効です。",
              });
            }
            parsing_lyrics = false;
          }
          _ => {
            return Err(InvalidCommand {
              line_num,
              reason: "start、break、end コマンドのみが有効です。",
            });
          }
        }
      }
      if let Some(caption) = caption_reg.captures(line) {
        if !parsing_lyrics {
          return Err(InvalidStatementDeifinition {
            line_num,
            reason: "キャプションの指定は歌詞定義の中のみ有効です。",
          });
        }
        let string = caption.get(0).unwrap().as_str();
        notes.push(Note::caption(line_time, string));
      }
      if let Some(property) = property_reg.captures(line) {
        if parsing_lyrics {
          return Err(InvalidPropertyDeifinition {
            line_num,
            reason: "プロパティの指定は歌詞定義の外のみ有効です。",
          });
        }
        if property.len() != 3 {
          return Err(InvalidPropertyDeifinition {
            line_num,
            reason: "プロパティの指定が正しくありません。",
          });
        }
        let key = property.get(1).unwrap().as_str();
        if !METADATA_KEYS.contains(&key) {
          return Err(InvalidPropertyDeifinition {
            line_num,
            reason: "未対応のプロパティです。",
          });
        }
        let value = property.get(2).unwrap().as_str();
        metadata.insert(key.to_owned(), value.to_owned());
      }
      line_num += 1;
    }
    Ok(())
    //Ok(Scoremap {})
  }
}
