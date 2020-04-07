use super::lexer::{Token, TokenContent};
use super::{Scoremap, ScoremapMetadata};
use crate::model::exp::minute_second::MinuteSecond;
use crate::model::exp::note::Note;
use crate::model::exp::sentence::Sentence;

#[derive(Debug)]
pub enum ScoremapParseError {
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

pub fn parse(
  mut tokens: &[Token],
) -> Result<Scoremap, ScoremapParseError> {
  use ScoremapParseError::*;

  let mut metadata = ScoremapMetadata::new();
  let mut notes: Vec<Note> = vec![];
  let mut parsing_lyrics = false;
  let mut line_minute_second = MinuteSecond::new();
  let mut parsed_japanese: Option<String> = None;

  while 1 <= tokens.len() {
    let Token { line_num, content } = &tokens[0];
    let line_num = *line_num;
    let line_time = line_minute_second.to_seconds();
    match tokens {
      [Token {
        content: TokenContent::Minutes(minutes),
        ..
      }, Token {
        content: TokenContent::Seconds(seconds),
        ..
      }, ..] => {
        // 分と秒の同時更新
        let new_time =
          line_minute_second.minutes(*minutes).seconds(*seconds);
        if new_time.to_seconds() <= line_time {
          // 同じかそれ以前の時間指定は無視
          tokens = &tokens[2..];
          continue;
        }
        line_minute_second = new_time;
        tokens = &tokens[2..];
        continue;
      }
      _ => {}
    }
    match content {
      TokenContent::Seconds(seconds) => {
        let new_time = line_minute_second.seconds(*seconds);
        if new_time.to_seconds() <= line_time {
          // 同じかそれ以前の時間指定は無視
          tokens = &tokens[1..];
          continue;
        }
        check_before_define_timing(line_num, parsing_lyrics)?;
        if let Some(last) = notes.last() {
          if last.time() != line_time {
            // 歌詞が定義されなかったので、空白ノーツを追加
            notes.push(Note::blank(line_time));
          }
        }
        if notes.len() == 0 {
          // 最初に空白ノーツを追加
          notes.push(Note::blank(line_time));
        }
        parsed_japanese = None;
        line_minute_second = new_time;
        tokens = &tokens[1..];
      }
      TokenContent::Command(command) => {
        match command.as_str() {
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
        };
        tokens = &tokens[1..];
      }
      TokenContent::Caption(caption) => {
        if !parsing_lyrics {
          return Err(InvalidStatementDefinition {
            line_num,
            reason: "キャプションの指定は歌詞定義の中のみ有効です。",
          });
        }
        notes.push(Note::caption(line_time, caption.as_str()));
        tokens = &tokens[1..];
      }
      TokenContent::Property { key, value } => {
        if parsing_lyrics {
          return Err(InvalidPropertyDeifinition {
            line_num,
            reason: "プロパティの指定は歌詞定義の外のみ有効です。",
          });
        }
        metadata.insert(key.clone(), value.clone());
        tokens = &tokens[1..];
      }
      TokenContent::Yomigana(yomigana) => {
        if let Some(ref lyrics) = parsed_japanese {
          notes.push(Note::sentence(
            line_time,
            Sentence::from(lyrics.as_str(), yomigana.clone()),
          ));
          parsed_japanese = None;
          tokens = &tokens[1..];
          continue;
        }
        return Err(InvalidStatementDefinition {
          line_num,
          reason: "読み仮名は歌詞より後にしてください。",
        });
      }
      TokenContent::Section(_) => {
        tokens = &tokens[1..];
      }
      TokenContent::Lyrics(lyrics) => {
        if let Some(prev_lyrics) = parsed_japanese {
          parsed_japanese =
            Some(format!("{}{}", prev_lyrics, lyrics));
          tokens = &tokens[1..];
          continue;
        }
        parsed_japanese = Some(lyrics.to_owned());
        tokens = &tokens[1..];
      }
      _ => unreachable!(),
    }
  }
  // 最後に空白ノーツを追加
  let line_time = line_minute_second.to_seconds();
  notes.push(Note::blank(line_time));

  Ok(Scoremap { metadata, notes })
}

fn check_before_define_timing(
  line_num: usize,
  parsing_lyrics: bool,
) -> Result<(), ScoremapParseError> {
  use ScoremapParseError::*;

  if !parsing_lyrics {
    return Err(InvalidTimingDeifinition {
      line_num,
      reason: "時間指定は歌詞定義の中のみ有効です。",
    });
  }
  Ok(())
}
