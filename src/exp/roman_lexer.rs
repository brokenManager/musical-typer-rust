#[derive(Debug)]
pub enum RomanParseError {
  IllegalHiragana(String),
}

#[derive(Debug, PartialEq)]
pub struct RomanStr(Vec<Vec<&'static str>>);

impl RomanStr {
  pub fn new(hiragana: &str) -> Result<RomanStr, RomanParseError> {
    let chars: Vec<char> = hiragana.chars().collect();
    let (mut parsed, mut rest) =
      Self::parse(vec![], chars.as_slice())?;
    while rest.len() != 0 {
      let res = Self::parse(parsed, rest)?;
      parsed = res.0;
      rest = res.1;
    }
    Ok(RomanStr(parsed))
  }

  fn parse<'a>(
    romans: Vec<Vec<&'static str>>,
    hiragana: &'a [char],
  ) -> Result<(Vec<Vec<&'static str>>, &'a [char]), RomanParseError>
  {
    use RomanParseError::*;
    Ok(match hiragana {
      // ['っ', 'か', rest @ ..] =>
      //   ([romans, vec![vec!["k", "ka"]]].concat(), rest),
      ['し', 'ゃ', rest @ ..] => {
        ([romans, vec![vec!["sha", "sya"]]].concat(), rest)
      }
      ['し', 'ゅ', rest @ ..] => {
        ([romans, vec![vec!["shu", "syu"]]].concat(), rest)
      }
      ['し', 'ょ', rest @ ..] => {
        ([romans, vec![vec!["sho", "syo"]]].concat(), rest)
      }
      ['ち', 'ゃ', rest @ ..] => {
        ([romans, vec![vec!["cha", "cya", "tya"]]].concat(), rest)
      }
      ['ち', 'ゅ', rest @ ..] => {
        ([romans, vec![vec!["chu", "cyu", "tyu"]]].concat(), rest)
      }
      ['ち', 'ょ', rest @ ..] => {
        ([romans, vec![vec!["cho", "cyo", "tyo"]]].concat(), rest)
      }

      ['じ', 'ゃ', rest @ ..] => {
        ([romans, vec![vec!["ja", "jya"]]].concat(), rest)
      }
      ['じ', 'ゅ', rest @ ..] => {
        ([romans, vec![vec!["ju", "jyu"]]].concat(), rest)
      }
      ['じ', 'ょ', rest @ ..] => {
        ([romans, vec![vec!["jo", "jyo"]]].concat(), rest)
      }
      ['ぢ', 'ゃ', rest @ ..] => {
        ([romans, vec![vec!["dya"]]].concat(), rest)
      }
      ['ぢ', 'ゅ', rest @ ..] => {
        ([romans, vec![vec!["dyu"]]].concat(), rest)
      }
      ['ぢ', 'ょ', rest @ ..] => {
        ([romans, vec![vec!["dyo"]]].concat(), rest)
      }

      ['あ', rest @ ..] => {
        ([romans, vec![vec!["a"]]].concat(), rest)
      }
      ['い', rest @ ..] => {
        ([romans, vec![vec!["i"]]].concat(), rest)
      }
      ['う', rest @ ..] => {
        ([romans, vec![vec!["u"]]].concat(), rest)
      }
      ['え', rest @ ..] => {
        ([romans, vec![vec!["e"]]].concat(), rest)
      }
      ['お', rest @ ..] => {
        ([romans, vec![vec!["o"]]].concat(), rest)
      }
      ['か', rest @ ..] => {
        ([romans, vec![vec!["ka"]]].concat(), rest)
      }
      ['き', rest @ ..] => {
        ([romans, vec![vec!["ki"]]].concat(), rest)
      }
      ['く', rest @ ..] => {
        ([romans, vec![vec!["ku"]]].concat(), rest)
      }
      ['け', rest @ ..] => {
        ([romans, vec![vec!["ke"]]].concat(), rest)
      }
      ['こ', rest @ ..] => {
        ([romans, vec![vec!["ko"]]].concat(), rest)
      }
      ['さ', rest @ ..] => {
        ([romans, vec![vec!["sa"]]].concat(), rest)
      }
      ['し', rest @ ..] => {
        ([romans, vec![vec!["si", "shi"]]].concat(), rest)
      }
      ['す', rest @ ..] => {
        ([romans, vec![vec!["su"]]].concat(), rest)
      }
      ['せ', rest @ ..] => {
        ([romans, vec![vec!["sw"]]].concat(), rest)
      }
      ['そ', rest @ ..] => {
        ([romans, vec![vec!["so"]]].concat(), rest)
      }
      ['た', rest @ ..] => {
        ([romans, vec![vec!["ta"]]].concat(), rest)
      }
      ['ち', rest @ ..] => {
        ([romans, vec![vec!["ti", "chi"]]].concat(), rest)
      }
      ['つ', rest @ ..] => {
        ([romans, vec![vec!["tu"]]].concat(), rest)
      }
      ['て', rest @ ..] => {
        ([romans, vec![vec!["te"]]].concat(), rest)
      }
      ['と', rest @ ..] => {
        ([romans, vec![vec!["to"]]].concat(), rest)
      }
      ['な', rest @ ..] => {
        ([romans, vec![vec!["na"]]].concat(), rest)
      }
      ['に', rest @ ..] => {
        ([romans, vec![vec!["ni"]]].concat(), rest)
      }
      ['ぬ', rest @ ..] => {
        ([romans, vec![vec!["nu"]]].concat(), rest)
      }
      ['ね', rest @ ..] => {
        ([romans, vec![vec!["ne"]]].concat(), rest)
      }
      ['の', rest @ ..] => {
        ([romans, vec![vec!["no"]]].concat(), rest)
      }
      ['は', rest @ ..] => {
        ([romans, vec![vec!["ha"]]].concat(), rest)
      }
      ['ひ', rest @ ..] => {
        ([romans, vec![vec!["hi"]]].concat(), rest)
      }
      ['ふ', rest @ ..] => {
        ([romans, vec![vec!["hu"]]].concat(), rest)
      }
      ['へ', rest @ ..] => {
        ([romans, vec![vec!["he"]]].concat(), rest)
      }
      ['ほ', rest @ ..] => {
        ([romans, vec![vec!["ho"]]].concat(), rest)
      }
      ['ま', rest @ ..] => {
        ([romans, vec![vec!["ma"]]].concat(), rest)
      }
      ['み', rest @ ..] => {
        ([romans, vec![vec!["mi"]]].concat(), rest)
      }
      ['む', rest @ ..] => {
        ([romans, vec![vec!["mu"]]].concat(), rest)
      }
      ['め', rest @ ..] => {
        ([romans, vec![vec!["me"]]].concat(), rest)
      }
      ['も', rest @ ..] => {
        ([romans, vec![vec!["mo"]]].concat(), rest)
      }
      ['や', rest @ ..] => {
        ([romans, vec![vec!["ya"]]].concat(), rest)
      }
      ['ゆ', rest @ ..] => {
        ([romans, vec![vec!["yu"]]].concat(), rest)
      }
      ['よ', rest @ ..] => {
        ([romans, vec![vec!["yo"]]].concat(), rest)
      }
      ['ら', rest @ ..] => {
        ([romans, vec![vec!["ra"]]].concat(), rest)
      }
      ['り', rest @ ..] => {
        ([romans, vec![vec!["ri"]]].concat(), rest)
      }
      ['る', rest @ ..] => {
        ([romans, vec![vec!["ru"]]].concat(), rest)
      }
      ['れ', rest @ ..] => {
        ([romans, vec![vec!["re"]]].concat(), rest)
      }
      ['ろ', rest @ ..] => {
        ([romans, vec![vec!["ro"]]].concat(), rest)
      }
      ['わ', rest @ ..] => {
        ([romans, vec![vec!["wa"]]].concat(), rest)
      }
      ['ゐ', rest @ ..] => {
        ([romans, vec![vec!["wi"]]].concat(), rest)
      }
      ['ゑ', rest @ ..] => {
        ([romans, vec![vec!["we"]]].concat(), rest)
      }
      ['を', rest @ ..] => {
        ([romans, vec![vec!["wo"]]].concat(), rest)
      }
      ['ん', rest @ ..] => {
        ([romans, vec![vec!["nn"]]].concat(), rest)
      }
      ['ぁ', rest @ ..] => {
        ([romans, vec![vec!["xa", "la"]]].concat(), rest)
      }
      ['ぃ', rest @ ..] => {
        ([romans, vec![vec!["xi", "li"]]].concat(), rest)
      }
      ['ぅ', rest @ ..] => {
        ([romans, vec![vec!["xu", "lu"]]].concat(), rest)
      }
      ['ぇ', rest @ ..] => {
        ([romans, vec![vec!["xe", "le"]]].concat(), rest)
      }
      ['ぉ', rest @ ..] => {
        ([romans, vec![vec!["xo", "lo"]]].concat(), rest)
      }
      ['っ', rest @ ..] => {
        ([romans, vec![vec!["xtu", "ltu"]]].concat(), rest)
      }

      ['が', rest @ ..] => {
        ([romans, vec![vec!["ga"]]].concat(), rest)
      }
      ['ぎ', rest @ ..] => {
        ([romans, vec![vec!["gi"]]].concat(), rest)
      }
      ['ぐ', rest @ ..] => {
        ([romans, vec![vec!["gu"]]].concat(), rest)
      }
      ['げ', rest @ ..] => {
        ([romans, vec![vec!["ge"]]].concat(), rest)
      }
      ['ご', rest @ ..] => {
        ([romans, vec![vec!["go"]]].concat(), rest)
      }
      ['ざ', rest @ ..] => {
        ([romans, vec![vec!["za"]]].concat(), rest)
      }
      ['じ', rest @ ..] => {
        ([romans, vec![vec!["zi", "ji"]]].concat(), rest)
      }
      ['ず', rest @ ..] => {
        ([romans, vec![vec!["zu"]]].concat(), rest)
      }
      ['ぜ', rest @ ..] => {
        ([romans, vec![vec!["ze"]]].concat(), rest)
      }
      ['ぞ', rest @ ..] => {
        ([romans, vec![vec!["zo"]]].concat(), rest)
      }
      ['だ', rest @ ..] => {
        ([romans, vec![vec!["da"]]].concat(), rest)
      }
      ['ぢ', rest @ ..] => {
        ([romans, vec![vec!["di"]]].concat(), rest)
      }
      ['づ', rest @ ..] => {
        ([romans, vec![vec!["du"]]].concat(), rest)
      }
      ['で', rest @ ..] => {
        ([romans, vec![vec!["de"]]].concat(), rest)
      }
      ['ど', rest @ ..] => {
        ([romans, vec![vec!["do"]]].concat(), rest)
      }
      ['ば', rest @ ..] => {
        ([romans, vec![vec!["ba"]]].concat(), rest)
      }
      ['び', rest @ ..] => {
        ([romans, vec![vec!["bi"]]].concat(), rest)
      }
      ['ぶ', rest @ ..] => {
        ([romans, vec![vec!["bu"]]].concat(), rest)
      }
      ['べ', rest @ ..] => {
        ([romans, vec![vec!["be"]]].concat(), rest)
      }
      ['ぼ', rest @ ..] => {
        ([romans, vec![vec!["bo"]]].concat(), rest)
      }
      ['ぱ', rest @ ..] => {
        ([romans, vec![vec!["pa"]]].concat(), rest)
      }
      ['ぴ', rest @ ..] => {
        ([romans, vec![vec!["pi"]]].concat(), rest)
      }
      ['ぷ', rest @ ..] => {
        ([romans, vec![vec!["pu"]]].concat(), rest)
      }
      ['ぺ', rest @ ..] => {
        ([romans, vec![vec!["pe"]]].concat(), rest)
      }
      ['ぽ', rest @ ..] => {
        ([romans, vec![vec!["po"]]].concat(), rest)
      }
      n => {
        return Err(IllegalHiragana(format!("{:#?}", n)));
      }
    })
  }

  pub fn exprs(&self) -> &Vec<Vec<&'static str>> {
    &self.0
  }
}

#[test]
fn jojo() {
  let lexed = RomanStr::new("そのちのさだめ").unwrap();
  assert_eq!(
    lexed.exprs(),
    &[
      vec!["so"],
      vec!["no"],
      vec!["ti", "chi"],
      vec!["no"],
      vec!["sa"],
      vec!["da"],
      vec!["me"],
    ]
  );
}
