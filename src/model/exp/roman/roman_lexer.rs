use super::roman_char::RomanChar;
use crate::model::exp::scoremap::lexer::ScoremapLexError;
use crate::model::exp::scoremap::ScoremapError;

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
      ['っ', 'く', 'ぁ' | 'ぃ' | 'ぅ' | 'ぇ' | 'ぉ', ..] => {
        romans.push(RomanChar::new(&["q", "k", "xtu", "ltu"]));
        1
      }
      ['っ', 'か' | 'き' | 'く' | 'け' | 'こ', ..] => {
        romans.push(RomanChar::new(&["k", "xtu", "ltu"]));
        1
      }
      ['っ', 'さ' | 'し' | 'す' | 'せ' | 'そ', ..] => {
        romans.push(RomanChar::new(&["s", "xtu", "ltu"]));
        1
      }
      ['っ', 'た' | 'つ' | 'て' | 'と', ..] => {
        romans.push(RomanChar::new(&["t", "xtu", "ltu"]));
        1
      }
      ['っ', 'ち', ..] => {
        romans.push(RomanChar::new(&["t", "c", "xtu", "ltu"]));
        1
      }
      ['っ', 'は' | 'ひ' | 'ふ' | 'へ' | 'ほ', ..] => {
        romans.push(RomanChar::new(&["h", "xtu", "ltu"]));
        1
      }
      ['っ', 'ま' | 'み' | 'む' | 'め' | 'も', ..] => {
        romans.push(RomanChar::new(&["m", "xtu", "ltu"]));
        1
      }
      ['っ', 'や' | 'ゆ' | 'よ', ..] => {
        romans.push(RomanChar::new(&["y", "xtu", "ltu"]));
        1
      }
      ['っ', 'ら' | 'り' | 'る' | 'れ' | 'ろ', ..] => {
        romans.push(RomanChar::new(&["r", "xtu", "ltu"]));
        1
      }
      ['っ', 'わ' | 'ゐ' | 'ゑ' | 'を', ..] => {
        romans.push(RomanChar::new(&["w", "xtu", "ltu"]));
        1
      }
      ['っ', 'ざ' | 'ず' | 'ぜ' | 'ぞ', ..] => {
        romans.push(RomanChar::new(&["z", "xtu", "ltu"]));
        1
      }
      ['っ', 'じ', ..] => {
        romans.push(RomanChar::new(&["z", "j", "xtu", "ltu"]));
        1
      }
      ['っ', 'だ' | 'ぢ' | 'づ' | 'で' | 'ど', ..] => {
        romans.push(RomanChar::new(&["d", "xtu", "ltu"]));
        1
      }
      ['っ', 'ば' | 'び' | 'ぶ' | 'べ' | 'ぼ', ..] => {
        romans.push(RomanChar::new(&["b", "xtu", "ltu"]));
        1
      }
      ['っ', 'ぱ' | 'ぴ' | 'ぷ' | 'ぺ' | 'ぽ', ..] => {
        romans.push(RomanChar::new(&["p", "xtu", "ltu"]));
        1
      }
      ['う', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["wi"]));
        2
      }
      ['う', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["we"]));
        2
      }
      ['き', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["kya"]));
        2
      }
      ['き', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["kyi"]));
        2
      }
      ['き', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["kyu"]));
        2
      }
      ['き', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["kye"]));
        2
      }
      ['き', 'ょ', ..] => {
        romans.push(RomanChar::new(&["kyo"]));
        2
      }
      ['ぎ', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["gya"]));
        2
      }
      ['ぎ', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["gyi"]));
        2
      }
      ['ぎ', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["gyu"]));
        2
      }
      ['ぎ', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["gye"]));
        2
      }
      ['ぎ', 'ょ', ..] => {
        romans.push(RomanChar::new(&["gyo"]));
        2
      }
      ['く', 'ぁ', ..] => {
        romans.push(RomanChar::new(&["qa", "kwa"]));
        2
      }
      ['く', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["qi", "kwi"]));
        2
      }
      ['く', 'ぅ', ..] => {
        romans.push(RomanChar::new(&["qu", "kwu"]));
        2
      }
      ['く', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["qe", "kwe"]));
        2
      }
      ['く', 'ぉ', ..] => {
        romans.push(RomanChar::new(&["qo", "kwo"]));
        2
      }
      ['し', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["sha", "sya"]));
        2
      }
      ['し', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["syi"]));
        2
      }
      ['し', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["shu", "syu"]));
        2
      }
      ['し', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["sye"]));
        2
      }
      ['し', 'ょ', ..] => {
        romans.push(RomanChar::new(&["sho", "syo"]));
        2
      }
      ['ち', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["tya", "cha", "cya"]));
        2
      }
      ['ち', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["cyi", "tyi"]));
        2
      }
      ['ち', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["chu", "cyu", "tyu"]));
        2
      }
      ['ち', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["cye", "tye"]));
        2
      }
      ['ち', 'ょ', ..] => {
        romans.push(RomanChar::new(&["cho", "cyo", "tyo"]));
        2
      }
      ['に', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["nya"]));
        2
      }
      ['に', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["nyi"]));
        2
      }
      ['に', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["nyu"]));
        2
      }
      ['に', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["nye"]));
        2
      }
      ['に', 'ょ', ..] => {
        romans.push(RomanChar::new(&["nyo"]));
        2
      }
      ['ひ', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["hya"]));
        2
      }
      ['ひ', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["hyi"]));
        2
      }
      ['ひ', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["hyu"]));
        2
      }
      ['ひ', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["hye"]));
        2
      }
      ['ひ', 'ょ', ..] => {
        romans.push(RomanChar::new(&["hyo"]));
        2
      }
      ['ふ', 'ぁ', ..] => {
        romans.push(RomanChar::new(&["fa"]));
        2
      }
      ['ふ', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["fi"]));
        2
      }
      ['ふ', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["fe"]));
        2
      }
      ['ふ', 'ぉ', ..] => {
        romans.push(RomanChar::new(&["fo"]));
        2
      }
      ['ふ', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["fya"]));
        2
      }
      ['ふ', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["fyu"]));
        2
      }
      ['ふ', 'ょ', ..] => {
        romans.push(RomanChar::new(&["fyo"]));
        2
      }
      ['み', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["mya"]));
        2
      }
      ['み', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["myi"]));
        2
      }
      ['み', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["myu"]));
        2
      }
      ['み', 'ょ', ..] => {
        romans.push(RomanChar::new(&["myo"]));
        2
      }
      ['り', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["rya"]));
        2
      }
      ['り', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["ryi"]));
        2
      }
      ['り', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["ryu"]));
        2
      }
      ['り', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["rye"]));
        2
      }
      ['り', 'ょ', ..] => {
        romans.push(RomanChar::new(&["ryo"]));
        2
      }
      ['ゔ', 'ぁ', ..] => {
        romans.push(RomanChar::new(&["va"]));
        2
      }
      ['ゔ', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["vi"]));
        2
      }
      ['ゔ', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["ve"]));
        2
      }
      ['ゔ', 'ぉ', ..] => {
        romans.push(RomanChar::new(&["vo"]));
        2
      }
      ['ゔ', ..] => {
        romans.push(RomanChar::new(&["vu"]));
        1
      }
      ['ぐ', 'ぁ', ..] => {
        romans.push(RomanChar::new(&["gwa"]));
        2
      }
      ['ぐ', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["gwi"]));
        2
      }
      ['ぐ', 'ぅ', ..] => {
        romans.push(RomanChar::new(&["gwu"]));
        2
      }
      ['ぐ', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["gwe"]));
        2
      }
      ['ぐ', 'ぉ', ..] => {
        romans.push(RomanChar::new(&["gwo"]));
        2
      }
      ['じ', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["ja", "jya"]));
        2
      }
      ['じ', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["jyi"]));
        2
      }
      ['じ', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["ju", "jyu"]));
        2
      }
      ['じ', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["jye"]));
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
      ['ぢ', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["dye"]));
        2
      }
      ['ぢ', 'ょ', ..] => {
        romans.push(RomanChar::new(&["dyo"]));
        2
      }
      ['び', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["bya"]));
        2
      }
      ['び', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["byi"]));
        2
      }
      ['び', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["byu"]));
        2
      }
      ['び', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["bye"]));
        2
      }
      ['び', 'ょ', ..] => {
        romans.push(RomanChar::new(&["hyo"]));
        2
      }
      ['ぴ', 'ゃ', ..] => {
        romans.push(RomanChar::new(&["pya"]));
        2
      }
      ['ぴ', 'ぃ', ..] => {
        romans.push(RomanChar::new(&["pyi"]));
        2
      }
      ['ぴ', 'ゅ', ..] => {
        romans.push(RomanChar::new(&["pyu"]));
        2
      }
      ['ぴ', 'ぇ', ..] => {
        romans.push(RomanChar::new(&["pye"]));
        2
      }
      ['ぴ', 'ょ', ..] => {
        romans.push(RomanChar::new(&["pyo"]));
        2
      }
      ['ん', 'な' | 'に' | 'ぬ' | 'ね' | 'の', ..] => {
        romans.push(RomanChar::new(&["nn"]));
        1
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
  for (expected, actual) in [
    vec!["so"],
    vec!["no"],
    vec!["ti", "chi"],
    vec!["no"],
    vec!["sa"],
    vec!["da"],
    vec!["me"],
  ]
  .iter()
  .zip(parsed.iter())
  {
    assert_eq!(expected.len(), actual.styles().len());
    for (expected, actual) in expected.iter().zip(actual.styles()) {
      assert_eq!(expected, actual);
    }
  }
  Ok(())
}

#[test]
fn panst() -> Result<(), RomanParseError> {
  let mut parsed: Vec<RomanChar> = vec![];
  parse(
    &mut parsed,
    "ぱんてぃーあんどすとっきんぐ"
      .chars()
      .collect::<Vec<char>>()
      .as_slice(),
  )?;
  for (expected, actual) in [
    vec!["pa"],
    vec!["n"],
    vec!["te"],
    vec!["xi", "li"],
    vec!["-"],
    vec!["a"],
    vec!["n"],
    vec!["do"],
    vec!["su"],
    vec!["to"],
    vec!["k", "xtu", "ltu"],
    vec!["ki"],
    vec!["n"],
    vec!["gu"],
  ]
  .iter()
  .zip(parsed.iter())
  {
    for (expected, actual) in expected.iter().zip(actual.styles()) {
      assert_eq!(expected, actual);
    }
    assert_eq!(expected.len(), actual.styles().len());
  }
  Ok(())
}

#[test]
fn ff12rw() -> Result<(), RomanParseError> {
  let mut parsed: Vec<RomanChar> = vec![];
  parse(
    &mut parsed,
    "ふぁいなるふぁんたじーとぅえるぶれゔぁなんとうぃんぐ"
      .chars()
      .collect::<Vec<char>>()
      .as_slice(),
  )?;
  for (expected, actual) in [
    vec!["fa"],
    vec!["i"],
    vec!["na"],
    vec!["ru"],
    vec!["fa"],
    vec!["n"],
    vec!["ta"],
    vec!["zi", "ji"],
    vec!["-"],
    vec!["to"],
    vec!["xu", "lu"],
    vec!["e"],
    vec!["ru"],
    vec!["bu"],
    vec!["re"],
    vec!["va"],
    vec!["na"],
    vec!["n"],
    vec!["to"],
    vec!["wi"],
    vec!["n"],
    vec!["gu"],
  ]
  .iter()
  .zip(parsed.iter())
  {
    for (expected, actual) in expected.iter().zip(actual.styles()) {
      assert_eq!(expected, actual);
    }
    assert_eq!(expected.len(), actual.styles().len());
  }
  Ok(())
}

#[test]
fn chocolate_balls() -> Result<(), RomanParseError> {
  let mut parsed: Vec<RomanChar> = vec![];
  parse(
    &mut parsed,
    "くぇっくぇっくぇっちょこぼーる"
      .chars()
      .collect::<Vec<char>>()
      .as_slice(),
  )?;
  for (expected, actual) in [
    vec!["qe", "kwe"],
    vec!["q", "k", "xtu", "ltu"],
    vec!["qe", "kwe"],
    vec!["q", "k", "xtu", "ltu"],
    vec!["qe", "kwe"],
    vec!["t", "c", "xtu", "ltu"],
    vec!["cho", "cyo", "tyo"],
    vec!["ko"],
    vec!["bo"],
    vec!["-"],
    vec!["ru"],
  ]
  .iter()
  .zip(parsed.iter())
  {
    for (expected, actual) in expected.iter().zip(actual.styles()) {
      assert_eq!(expected, actual);
    }
    assert_eq!(expected.len(), actual.styles().len());
  }
  Ok(())
}
