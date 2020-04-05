use std::collections::HashMap;

use super::minute_second::MinuteSecond;
use super::note::Note;
use super::sentence::Sentence;

pub mod lexer;

use lexer::{ScoremapLexError, Token, TokenContent};

#[derive(Debug)]
pub enum ScoremapError {
  LexError(ScoremapLexError),
  UnexceptedEndOfFile,
  InvalidCommand {
    line_num: usize,
    reason: &'static str,
  },
  InvalidPropertyDeifinition {
    line_num: usize,
    reason: &'static str,
  },
  InvalidStatementDefinition {
    line_num: usize,
    reason: &'static str,
  },
  InvalidTimingDeifinition {
    line_num: usize,
    reason: &'static str,
  },
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

    let mut metadata = ScoremapMetadata::new();
    let mut notes: Vec<Note> = vec![];
    let mut parsing_lyrics = false;
    let mut line_minute_second = MinuteSecond::new();
    let mut parsed_japanese: Option<String> = None;

    use std::io::BufReader;
    let reader = BufReader::new(file);
    let tokens =
      lexer::lex(config, reader).map_err(|e| LexError(e))?;

    for token in tokens.into_iter() {
      let Token { line_num, content } = token;
      let line_time = line_minute_second.into_time();
      match content {
        TokenContent::Seconds(seconds) => {
          let new_time = line_minute_second.seconds(seconds);
          if new_time.into_time() == line_time {
            continue;
          }
          Self::check_before_define_timing(line_num, parsing_lyrics)?;

          parsed_japanese = None;
          line_minute_second = new_time;
        }
        TokenContent::Minutes(minutes) => {
          let new_time = line_minute_second.minutes(minutes);
          if new_time.into_time() == line_time {
            continue;
          }
          Self::check_before_define_timing(line_num, parsing_lyrics)?;

          parsed_japanese = None;
          line_minute_second = new_time;
        }
        TokenContent::Command(command) => match command.as_str() {
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
        },
        TokenContent::Caption(caption) => {
          if !parsing_lyrics {
            return Err(InvalidStatementDefinition {
              line_num,
              reason:
                "キャプションの指定は歌詞定義の中のみ有効です。",
            });
          }
          notes.push(Note::caption(line_time, caption.as_str()));
        }
        TokenContent::Property { key, value } => {
          if parsing_lyrics {
            return Err(InvalidPropertyDeifinition {
              line_num,
              reason: "プロパティの指定は歌詞定義の外のみ有効です。",
            });
          }
          metadata.insert(key, value);
        }
        TokenContent::Yomigana(yomigana) => {
          if let Some(ref lyrics) = parsed_japanese {
            notes.push(Note::sentence(
              line_time,
              Sentence::from(lyrics.as_str(), yomigana),
            ));
            parsed_japanese = None;
            continue;
          }
          return Err(InvalidStatementDefinition {
            line_num,
            reason: "読み仮名は歌詞より後にしてください。",
          });
        }
        TokenContent::Section(_) => {}
        TokenContent::Lyrics(lyrics) => {
          if let Some(_) = parsed_japanese {
            return Err(InvalidStatementDefinition {
              line_num,
              reason: "歌詞は複数行に分けないでください。",
            });
          }
          parsed_japanese = Some(lyrics.to_owned());
        }
        _ => {}
      }
    }
    Ok(Scoremap { metadata, notes })
  }

  fn check_before_define_timing(
    line_num: usize,
    parsing_lyrics: bool,
  ) -> Result<(), ScoremapError> {
    use ScoremapError::*;

    if !parsing_lyrics {
      return Err(InvalidTimingDeifinition {
        line_num,
        reason: "時間指定は歌詞定義の中のみ有効です。",
      });
    }
    Ok(())
  }
}
