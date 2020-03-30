use regex::Regex;

use super::note::Note;

#[derive(Debug)]
pub enum ScoremapError {
  UnexceptedEndOfFile,
  InvalidCommand { line_num: u64, reason: &'static str },
  InvalidStatementDeifinition { line_num: u64, reason: &'static str },
}

pub struct ScoremapMetadata {}

pub struct Scoremap {
  metadata: ScoremapMetadata,
  notes: Vec<Note>,
}

const COMMENT: &str = r"^[[:space:]]*#?.*$";
const COMMAND: &str =
  r"^[[:space:]]*\[[[:space:]]*(.*)[[:space:]]*\][[:space:]]*$";
const HIRAGANA: &str = r"^:([あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわゐゑをんぁぃぅぇぉゃゅょゎっーがぎぐげござじずぜぞだぢづでどばびぶべぼぱぴぷぺぽ]+)$";
const CAPTION: &str = r"^>>(.+)$";

#[test]
fn pattern_tests() {
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

    let comment_reg = Regex::new(COMMENT).unwrap();
    let command_reg = Regex::new(COMMAND).unwrap();
    let hiragana_reg = Regex::new(HIRAGANA).unwrap();
    let caption_reg = Regex::new(CAPTION).unwrap();

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
      line_num += 1;
    }
    Ok(())
    //Ok(Scoremap {})
  }
}
