use super::roman_char::RomanChar;
use crate::exp::scoremap::lexer::ScoremapLexError;
use crate::exp::scoremap::ScoremapError;

#[derive(Debug)]
pub enum RomanParseError {
  IllegalYomigana(String),
}

impl From<RomanParseError> for ScoremapError {
  fn from(_err: RomanParseError) -> Self {
    ScoremapError::LexError(
      ScoremapLexError::InvalidStatementDefinition {
        line_num: 1,
        reason: "ふりがなでのそのような平仮名の並びは未対応です。",
      },
    )
  }
}

pub fn parse<'a>(
  romans: &mut Vec<RomanChar>,
  mut yomigana: &'a [char],
) -> Result<(), RomanParseError> {
  while yomigana.len() != 0 {
    let replaced_count = match yomigana {
      // ['っ', 'か', ..] => {
      //   romans.push(RomanChar::new(&["k", "ka"]));
      //   2
      // }
      // :
      // :
      // ['ゔ', 'ぁ', ..] => {
      //   romans.push(RomanChar::new(&["va"]));
      //   2
      // }
      // :
      // :
      ['ん', 'な' | 'に' | 'ぬ' | 'ね' | 'の', ..] => {
        romans.push(RomanChar::new(&["nn"]));
        1
      }

      ['し', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["sha", "sya"]));
        2
      }
      ['し', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["shu", "syu"]));
        2
      }
      ['し', 'ょ', ..] => {
        romans.push(RomanChar::new(&["sho", "syo"]));
        2
      }
      ['ち', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["cha", "cya", "tya"]));
        2
      }
      ['ち', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["chu", "cyu", "tyu"]));
        2
      }
      ['ち', 'ょ', ..] => {
        romans.push(RomanChar::new(&["cho", "cyo", "tyo"]));
        2
      }

      ['じ', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["ja", "jya"]));
        2
      }
      ['じ', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["ju", "jyu"]));
        2
      }
      ['じ', 'ょ', ..] => {
        romans.push(RomanChar::new(&["jo", "jyo"]));
        2
      }
      ['ぢ', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["dya"]));
        2
      }
      ['ぢ', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["dyu"]));
        2
      }
      ['ぢ', 'ょ', ..] => {
        romans.push(RomanChar::new(&["dyo"]));
        2
      }

      ['あ', ..] => {
        romans.push(RomanChar::new(&["a"]));
        1
      }
      ['い', ..] => {
        romans.push(RomanChar::new(&["i"]));
        1
      }
      ['う', ..] => {
        romans.push(RomanChar::new(&["u"]));
        1
      }
      ['え', ..] => {
        romans.push(RomanChar::new(&["e"]));
        1
      }
      ['お', ..] => {
        romans.push(RomanChar::new(&["o"]));
        1
      }
      ['か', ..] => {
        romans.push(RomanChar::new(&["ka"]));
        1
      }
      ['き', ..] => {
        romans.push(RomanChar::new(&["ki"]));
        1
      }
      ['く', ..] => {
        romans.push(RomanChar::new(&["ku"]));
        1
      }
      ['け', ..] => {
        romans.push(RomanChar::new(&["ke"]));
        1
      }
      ['こ', ..] => {
        romans.push(RomanChar::new(&["ko"]));
        1
      }
      ['さ', ..] => {
        romans.push(RomanChar::new(&["sa"]));
        1
      }
      ['し', ..] => {
        romans.push(RomanChar::new(&["si", "shi"]));
        1
      }
      ['す', ..] => {
        romans.push(RomanChar::new(&["su"]));
        1
      }
      ['せ', ..] => {
        romans.push(RomanChar::new(&["se"]));
        1
      }
      ['そ', ..] => {
        romans.push(RomanChar::new(&["so"]));
        1
      }
      ['た', ..] => {
        romans.push(RomanChar::new(&["ta"]));
        1
      }
      ['ち', ..] => {
        romans.push(RomanChar::new(&["ti", "chi"]));
        1
      }
      ['つ', ..] => {
        romans.push(RomanChar::new(&["tu"]));
        1
      }
      ['て', ..] => {
        romans.push(RomanChar::new(&["te"]));
        1
      }
      ['と', ..] => {
        romans.push(RomanChar::new(&["to"]));
        1
      }
      ['な', ..] => {
        romans.push(RomanChar::new(&["na"]));
        1
      }
      ['に', ..] => {
        romans.push(RomanChar::new(&["ni"]));
        1
      }
      ['ぬ', ..] => {
        romans.push(RomanChar::new(&["nu"]));
        1
      }
      ['ね', ..] => {
        romans.push(RomanChar::new(&["ne"]));
        1
      }
      ['の', ..] => {
        romans.push(RomanChar::new(&["no"]));
        1
      }
      ['は', ..] => {
        romans.push(RomanChar::new(&["ha"]));
        1
      }
      ['ひ', ..] => {
        romans.push(RomanChar::new(&["hi"]));
        1
      }
      ['ふ', ..] => {
        romans.push(RomanChar::new(&["hu"]));
        1
      }
      ['へ', ..] => {
        romans.push(RomanChar::new(&["he"]));
        1
      }
      ['ほ', ..] => {
        romans.push(RomanChar::new(&["ho"]));
        1
      }
      ['ま', ..] => {
        romans.push(RomanChar::new(&["ma"]));
        1
      }
      ['み', ..] => {
        romans.push(RomanChar::new(&["mi"]));
        1
      }
      ['む', ..] => {
        romans.push(RomanChar::new(&["mu"]));
        1
      }
      ['め', ..] => {
        romans.push(RomanChar::new(&["me"]));
        1
      }
      ['も', ..] => {
        romans.push(RomanChar::new(&["mo"]));
        1
      }
      ['や', ..] => {
        romans.push(RomanChar::new(&["ya"]));
        1
      }
      ['ゆ', ..] => {
        romans.push(RomanChar::new(&["yu"]));
        1
      }
      ['よ', ..] => {
        romans.push(RomanChar::new(&["yo"]));
        1
      }
      ['ら', ..] => {
        romans.push(RomanChar::new(&["ra"]));
        1
      }
      ['り', ..] => {
        romans.push(RomanChar::new(&["ri"]));
        1
      }
      ['る', ..] => {
        romans.push(RomanChar::new(&["ru"]));
        1
      }
      ['れ', ..] => {
        romans.push(RomanChar::new(&["re"]));
        1
      }
      ['ろ', ..] => {
        romans.push(RomanChar::new(&["ro"]));
        1
      }
      ['わ', ..] => {
        romans.push(RomanChar::new(&["wa"]));
        1
      }
      ['ゐ', ..] => {
        romans.push(RomanChar::new(&["wi"]));
        1
      }
      ['ゑ', ..] => {
        romans.push(RomanChar::new(&["we"]));
        1
      }
      ['を', ..] => {
        romans.push(RomanChar::new(&["wo"]));
        1
      }
      ['ん', ..] => {
        romans.push(RomanChar::new(&["n"]));
        1
      }
      ['ぁ', ..] => {
        romans.push(RomanChar::new(&["xa", "la"]));
        1
      }
      ['ぃ', ..] => {
        romans.push(RomanChar::new(&["xi", "li"]));
        1
      }
      ['ぅ', ..] => {
        romans.push(RomanChar::new(&["xu", "lu"]));
        1
      }
      ['ぇ', ..] => {
        romans.push(RomanChar::new(&["xe", "le"]));
        1
      }
      ['ぉ', ..] => {
        romans.push(RomanChar::new(&["xo", "lo"]));
        1
      }
      ['っ', ..] => {
        romans.push(RomanChar::new(&["xtu", "ltu"]));
        1
      }
      ['ゃ', ..] => {
        romans.push(RomanChar::new(&["xya", "lya"]));
        1
      }
      ['ゅ', ..] => {
        romans.push(RomanChar::new(&["xyu", "lyu"]));
        1
      }
      ['ょ', ..] => {
        romans.push(RomanChar::new(&["xyo", "lyo"]));
        1
      }
      ['ゎ', ..] => {
        romans.push(RomanChar::new(&["xwa", "lwa"]));
        1
      }

      ['が', ..] => {
        romans.push(RomanChar::new(&["ga"]));
        1
      }
      ['ぎ', ..] => {
        romans.push(RomanChar::new(&["gi"]));
        1
      }
      ['ぐ', ..] => {
        romans.push(RomanChar::new(&["gu"]));
        1
      }
      ['げ', ..] => {
        romans.push(RomanChar::new(&["ge"]));
        1
      }
      ['ご', ..] => {
        romans.push(RomanChar::new(&["go"]));
        1
      }
      ['ざ', ..] => {
        romans.push(RomanChar::new(&["za"]));
        1
      }
      ['じ', ..] => {
        romans.push(RomanChar::new(&["zi", "ji"]));
        1
      }
      ['ず', ..] => {
        romans.push(RomanChar::new(&["zu"]));
        1
      }
      ['ぜ', ..] => {
        romans.push(RomanChar::new(&["ze"]));
        1
      }
      ['ぞ', ..] => {
        romans.push(RomanChar::new(&["zo"]));
        1
      }
      ['だ', ..] => {
        romans.push(RomanChar::new(&["da"]));
        1
      }
      ['ぢ', ..] => {
        romans.push(RomanChar::new(&["di"]));
        1
      }
      ['づ', ..] => {
        romans.push(RomanChar::new(&["du"]));
        1
      }
      ['で', ..] => {
        romans.push(RomanChar::new(&["de"]));
        1
      }
      ['ど', ..] => {
        romans.push(RomanChar::new(&["do"]));
        1
      }
      ['ば', ..] => {
        romans.push(RomanChar::new(&["ba"]));
        1
      }
      ['び', ..] => {
        romans.push(RomanChar::new(&["bi"]));
        1
      }
      ['ぶ', ..] => {
        romans.push(RomanChar::new(&["bu"]));
        1
      }
      ['べ', ..] => {
        romans.push(RomanChar::new(&["be"]));
        1
      }
      ['ぼ', ..] => {
        romans.push(RomanChar::new(&["bo"]));
        1
      }
      ['ぱ', ..] => {
        romans.push(RomanChar::new(&["pa"]));
        1
      }
      ['ぴ', ..] => {
        romans.push(RomanChar::new(&["pi"]));
        1
      }
      ['ぷ', ..] => {
        romans.push(RomanChar::new(&["pu"]));
        1
      }
      ['ぺ', ..] => {
        romans.push(RomanChar::new(&["pe"]));
        1
      }
      ['ぽ', ..] => {
        romans.push(RomanChar::new(&["po"]));
        1
      }

      ['ー', ..] => {
        romans.push(RomanChar::new(&["-"]));
        1
      }
      n => {
        return Err(RomanParseError::IllegalYomigana(format!(
          "{:#?}",
          n
        )));
      }
    };
    yomigana = yomigana.split_at(replaced_count).1;
  }
  Ok(())
}

#[test]
fn jojo() -> Result<(), RomanParseError> {
  let mut parsed: Vec<RomanChar> = vec![];
  parse(
    &mut parsed,
    "そのちのさだめ".chars().collect::<Vec<char>>().as_slice(),
  )?;
  assert!(parsed
    .iter()
    .zip(&[
      vec!["so"],
      vec!["no"],
      vec!["ti", "chi"],
      vec!["no"],
      vec!["sa"],
      vec!["da"],
      vec!["me"],
    ])
    .all(|(roman_char, except)| {
      let styles = roman_char.styles();
      if styles.len() != except.len() {
        return false;
      }
      for i in 0..styles.len() {
        if styles[i] != except[i] {
          return false;
        }
      }
      true
    }));
  Ok(())
}
