use super::ScoremapParseError;
use crate::model::exp::{
  note::Note,
  scoremap::{
    token::{Token, TokenContent},
    ScoremapMetadata,
  },
  sentence::Sentence,
  time::{Duration, MinuteSecond},
};
use std::collections::VecDeque;
use ScoremapParseError::*;

#[derive(Debug, Clone)]
pub struct ParserCtx {
  pub metadata: ScoremapMetadata,
  pub sections: Vec<Vec<Note>>,
  pub notes: Vec<Note>,
  parsing_lyrics: bool,
  parsed_japanese: Option<String>,
  curr_time: MinuteSecond,
}

impl ParserCtx {
  pub fn new() -> Self {
    Self {
      metadata: ScoremapMetadata::new(),
      sections: vec![],
      notes: vec![],
      parsing_lyrics: false,
      parsed_japanese: None,
      curr_time: MinuteSecond::new(),
    }
  }
  fn calc_duration(
    &self,
    tokens: &VecDeque<&Token>,
    line_num: usize,
  ) -> Result<Duration, ScoremapParseError> {
    let mut next_times =
      tokens.iter().flat_map(|token| match token.content {
        TokenContent::Time(ref mise) => vec![mise.clone()],
        _ => vec![],
      });
    let next_time = next_times.next().unwrap_or_else(|| {
      self
        .curr_time
        .clone()
        .seconds(self.curr_time.to_seconds() + 1.0.into())
    });
    Duration::new(
      self.curr_time.clone().to_seconds().as_f64(),
      next_time.to_seconds().as_f64(),
    )
    .map_err(|err| ScoremapParseError::InvalidDuration {
      line_num,
      err,
    })
  }
}

pub type ParserBody = fn(
  &mut VecDeque<&Token>,
  &mut ParserCtx,
) -> Option<Result<Note, ScoremapParseError>>;

pub fn double_time_processor(
  tokens: &mut VecDeque<&Token>,
  _: &mut ParserCtx,
) -> Option<Result<Note, ScoremapParseError>> {
  if let [Token {
    content: TokenContent::Time(from),
    ..
  }, Token {
    content: TokenContent::Time(to),
    ..
  }, ..] = &tokens.iter().collect::<Vec<_>>().as_slice()
  {
    tokens.remove(0);
    // 時間指定が二連続の場合は空白ノーツを追加
    let res = Some(Ok(Note::blank(
      Duration::new(
        from.to_seconds().as_f64(),
        to.to_seconds().as_f64(),
      )
      .ok()?,
    )));
    return res;
  }
  None
}

pub fn single_time_processor(
  tokens: &mut VecDeque<&Token>,
  ctx: &mut ParserCtx,
) -> Option<Result<Note, ScoremapParseError>> {
  if let Some(Token {
    content: TokenContent::Time(specified),
    line_num,
    ..
  }) = tokens.front()
  {
    let duration = ctx.calc_duration(tokens, *line_num).ok()?;
    tokens.remove(0);
    let ParserCtx {
      parsing_lyrics,
      curr_time,
      notes,
      parsed_japanese,
      ..
    } = ctx;
    if !*parsing_lyrics {
      eprintln!("{:?}", tokens);
      return Some(Err(InvalidTimingDeifinition {
        line_num: *line_num,
        reason: "時間指定は歌詞定義の中のみ有効です。",
      }));
    }
    if specified <= curr_time {
      // それ以前に遡る時間指定は無視
      return None;
    }
    *curr_time = specified.clone();
    *parsed_japanese = None;
    if notes.is_empty() {
      return Some(Ok(Note::blank(duration)));
    }
  }
  None
}

pub fn command_processor(
  tokens: &mut VecDeque<&Token>,
  ParserCtx { parsing_lyrics, .. }: &mut ParserCtx,
) -> Option<Result<Note, ScoremapParseError>> {
  if let Some(Token {
    content: TokenContent::Command(command),
    line_num,
    ..
  }) = tokens.front()
  {
    tokens.remove(0);
    let line_num = *line_num;
    match command.as_str() {
      "start" => {
        if *parsing_lyrics {
          return Some(Err(InvalidCommand {
            line_num,
            reason: "start コマンドは end コマンドより前で有効です。",
          }));
        }
        *parsing_lyrics = true;
      }
      "break" => {}
      "end" => {
        if !*parsing_lyrics {
          return Some(Err(InvalidCommand {
            line_num,
            reason: "end コマンドは start コマンドより後で有効です。",
          }));
        }
        *parsing_lyrics = false;
      }
      _ => {
        return Some(Err(InvalidCommand {
          line_num,
          reason: "start、break、end コマンドのみが有効です。",
        }));
      }
    }
  }
  None
}

pub fn caption_processor(
  tokens: &mut VecDeque<&Token>,
  ctx: &mut ParserCtx,
) -> Option<Result<Note, ScoremapParseError>> {
  if let Some(Token {
    content: TokenContent::Caption(caption),
    line_num,
  }) = tokens.front()
  {
    let duration = ctx.calc_duration(tokens, *line_num).ok()?;
    tokens.remove(0);
    let ParserCtx { parsing_lyrics, .. } = ctx;
    if !*parsing_lyrics {
      return Some(Err(InvalidStatementDefinition {
        line_num: *line_num,
        reason: "キャプションの指定は歌詞定義の中のみ有効です。",
      }));
    }
    return Some(Ok(Note::caption(duration, caption.as_str())));
  }
  None
}

pub fn property_processor(
  tokens: &mut VecDeque<&Token>,
  ParserCtx {
    metadata,
    parsing_lyrics,
    ..
  }: &mut ParserCtx,
) -> Option<Result<Note, ScoremapParseError>> {
  if let Some(Token {
    content: TokenContent::Property { key, value },
    line_num,
  }) = tokens.front()
  {
    tokens.remove(0);
    if *parsing_lyrics {
      return Some(Err(InvalidPropertyDeifinition {
        line_num: *line_num,
        reason: "プロパティの指定は歌詞定義の外のみ有効です。",
      }));
    }
    metadata.0.insert(key.clone(), value.clone());
  }
  None
}

pub fn yomigana_processor(
  tokens: &mut VecDeque<&Token>,
  ctx: &mut ParserCtx,
) -> Option<Result<Note, ScoremapParseError>> {
  if let Some(Token {
    content: TokenContent::Yomigana(yomigana),
    line_num,
  }) = tokens.front()
  {
    let duration = ctx.calc_duration(tokens, *line_num).ok()?;
    tokens.remove(0);
    let ParserCtx {
      parsed_japanese, ..
    } = ctx;
    if let Some(lyrics) = parsed_japanese {
      let sentence =
        Sentence::from(lyrics.as_str(), yomigana.clone());
      *parsed_japanese = None;
      return Some(Ok(Note::sentence(duration, sentence)));
    }
    return Some(Err(InvalidStatementDefinition {
      line_num: *line_num,
      reason: "読み仮名は歌詞より後にしてください。",
    }));
  }
  None
}

pub fn section_processor(
  tokens: &mut VecDeque<&Token>,
  ParserCtx {
    notes, sections, ..
  }: &mut ParserCtx,
) -> Option<Result<Note, ScoremapParseError>> {
  if let Some(Token {
    content: TokenContent::Section(_),
    ..
  }) = tokens.front()
  {
    tokens.remove(0);
    if !notes.is_empty() {
      sections.push(notes.clone());
      *notes = vec![];
    }
  }
  None
}

pub fn lyrics_processor(
  tokens: &mut VecDeque<&Token>,
  ParserCtx {
    parsed_japanese, ..
  }: &mut ParserCtx,
) -> Option<Result<Note, ScoremapParseError>> {
  if let Some(Token {
    content: TokenContent::Lyrics(lyrics),
    ..
  }) = tokens.front()
  {
    tokens.remove(0);
    *parsed_japanese = if let Some(prev_lyrics) = parsed_japanese {
      Some(format!("{}{}", prev_lyrics, lyrics))
    } else {
      Some(lyrics.into())
    }
  }
  None
}

pub fn comment_processor(
  tokens: &mut VecDeque<&Token>,
  _: &mut ParserCtx,
) -> Option<Result<Note, ScoremapParseError>> {
  if let Some(Token {
    content: TokenContent::Comment,
    ..
  }) = tokens.front()
  {
    tokens.remove(0);
  }
  None
}
