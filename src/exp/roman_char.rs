pub enum RomanParseError {
  IllegalHiragana,
}

pub struct RomanChar {
  exprs: Vec<&'static str>,
}

impl RomanChar {
  fn new(hiragana: &str) -> Result<RomanChar, RomanParseError> {
    use RomanParseError::*;

    Ok(RomanChar {
      exprs: match hiragana {
        "しゃ" => vec!["sha", "sya"],
        "しゅ" => vec!["shu", "syu"],
        "しょ" => vec!["sho", "syo"],
        "ちゃ" => vec!["cha", "cya", "tya"],
        "ちゅ" => vec!["chu", "cyu", "tyu"],
        "ちょ" => vec!["cho", "cyo", "tyo"],

        "あ" => vec!["a"],
        "い" => vec!["i"],
        "う" => vec!["u"],
        "え" => vec!["e"],
        "お" => vec!["o"],
        "か" => vec!["ka"],
        "き" => vec!["ki"],
        "く" => vec!["ku"],
        "け" => vec!["ke"],
        "こ" => vec!["ko"],
        "さ" => vec!["sa"],
        "し" => vec!["si", "shi"],
        "す" => vec!["su"],
        "せ" => vec!["se"],
        "そ" => vec!["so"],
        "た" => vec!["ta"],
        "ち" => vec!["ti", "chi"],
        "つ" => vec!["tu"],
        "て" => vec!["te"],
        "と" => vec!["to"],
        "な" => vec!["na"],
        "に" => vec!["ni"],
        "ぬ" => vec!["nu"],
        "ね" => vec!["ne"],
        "の" => vec!["no"],
        "ま" => vec!["ma"],
        "み" => vec!["mi"],
        "む" => vec!["mu"],
        "め" => vec!["me"],
        "も" => vec!["mo"],
        "や" => vec!["ya"],
        "ゆ" => vec!["yu"],
        "よ" => vec!["yo"],
        "ら" => vec!["ra"],
        "り" => vec!["ri"],
        "る" => vec!["ru"],
        "れ" => vec!["re"],
        "ろ" => vec!["ro"],
        "わ" => vec!["wa"],
        "ゐ" => vec!["wi"],
        "ゑ" => vec!["we"],
        "を" => vec!["wo"],
        "ん" => vec!["n"],
        "ぁ" => vec!["xa", "la"],
        "ぃ" => vec!["xi", "li"],
        "ぅ" => vec!["xu", "lu"],
        "ぇ" => vec!["xe", "le"],
        "ぉ" => vec!["xo", "lo"],
        _ => {
          return Err(IllegalHiragana);
        }
      },
    })
  }
}
