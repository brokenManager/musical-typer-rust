use super::{
  super::super::section::note::sentence::{
    roman::roman_str::RomanStr, Sentence,
  },
  parse, Note, ScoremapParseError, Token, TokenContent,
};
use crate::model::exp::{
  scoremap::{sections::Sections, Scoremap, ScoremapMetadata},
  time::{Duration, MinuteSecond},
};
use std::collections::HashMap;

#[test]
fn case1() -> Result<(), ScoremapParseError> {
  use TokenContent::*;
  let input = vec![
    Token {
      line_num: 1,
      content: Comment,
    },
    Token {
      line_num: 2,
      content: Comment,
    },
    Token {
      line_num: 3,
      content: Property {
        key: "title".into(),
        value: "満点星の約束".into(),
      },
    },
    Token {
      line_num: 4,
      content: Property {
        key: "song_author".into(),
        value: "Mikuro さいな".into(),
      },
    },
    Token {
      line_num: 5,
      content: Property {
        key: "singer".into(),
        value: "塩音ルト CERANA".into(),
      },
    },
    Token {
      line_num: 6,
      content: Property {
        key: "score_author".into(),
        value: "Mikuro さいな".into(),
      },
    },
    Token {
      line_num: 7,
      content: Property {
        key: "song_data".into(),
        value: "twinkle-grace.ogg".into(),
      },
    },
    Token {
      line_num: 8,
      content: Property {
        key: "bpm".into(),
        value: "200".into(),
      },
    },
    Token {
      line_num: 9,
      content: Comment,
    },
    Token {
      line_num: 10,
      content: Command("start".into()),
    },
    Token {
      line_num: 11,
      content: Lyrics("*0.0".into()),
    },
    Token {
      line_num: 12,
      content: Section("一番".into()),
    },
    Token {
      line_num: 13,
      content: Caption("満点星の約束".into()),
    },
    Token {
      line_num: 14,
      content: Comment,
    },
    Token {
      line_num: 15,
      content: Time(MinuteSecond::new().seconds(18.6)),
    },
    Token {
      line_num: 16,
      content: Section("Aメロ".into()),
    },
    Token {
      line_num: 17,
      content: Lyrics("体が浮くような 3000m の星空".into()),
    },
    Token {
      line_num: 18,
      content: Yomigana(
        RomanStr::new("からだがうくようなさんぜんめーとるのほしぞら")
          .unwrap(),
      ),
    },
    Token {
      line_num: 19,
      content: Comment,
    },
    Token {
      line_num: 20,
      content: Time(MinuteSecond::new().seconds(23.5)),
    },
    Token {
      line_num: 21,
      content: Lyrics("初めて来た日に 交わした願いが光る".into()),
    },
    Token {
      line_num: 22,
      content: Yomigana(
        RomanStr::new("はじめてきたひにかわしたねがいがひかる")
          .unwrap(),
      ),
    },
    Token {
      line_num: 23,
      content: Comment,
    },
    Token {
      line_num: 24,
      content: Time(MinuteSecond::new().seconds(28.5)),
    },
    Token {
      line_num: 25,
      content: Lyrics("遠くの街の光が 煌めきを隠して".into()),
    },
    Token {
      line_num: 26,
      content: Yomigana(
        RomanStr::new("とおくのまちのひかりがきらめきをかくして")
          .unwrap(),
      ),
    },
    Token {
      line_num: 27,
      content: Comment,
    },
    Token {
      line_num: 28,
      content: Time(MinuteSecond::new().seconds(33.3)),
    },
    Token {
      line_num: 29,
      content: Lyrics("今夜も少しずつ 希望が融けてゆく".into()),
    },
    Token {
      line_num: 30,
      content: Yomigana(
        RomanStr::new("こんやもすこしきぼうずつきぼうがとけてゆく")
          .unwrap(),
      ),
    },
    Token {
      line_num: 31,
      content: Comment,
    },
    Token {
      line_num: 32,
      content: Time(MinuteSecond::new().seconds(38.0)),
    },
    Token {
      line_num: 33,
      content: Section("Bメロ".into()),
    },
    Token {
      line_num: 34,
      content: Lyrics(
        "ああ僕ひとりの力じゃ 何にもできないだろうさ".into(),
      ),
    },
    Token {
      line_num: 35,
      content: Yomigana(
        RomanStr::new(
          "ああぼくひとりのちからじゃなんにもできないだろうさ",
        )
        .unwrap(),
      ),
    },
    Token {
      line_num: 36,
      content: Comment,
    },
    Token {
      line_num: 37,
      content: Time(MinuteSecond::new().seconds(42.7)),
    },
    Token {
      line_num: 38,
      content: Lyrics("築いた繋がりなら 宇宙を越えられる".into()),
    },
    Token {
      line_num: 39,
      content: Yomigana(
        RomanStr::new("きづいたつながりならうちゅうをこえられる")
          .unwrap(),
      ),
    },
    Token {
      line_num: 40,
      content: Comment,
    },
    Token {
      line_num: 41,
      content: Time(MinuteSecond::new().seconds(47.6)),
    },
    Token {
      line_num: 42,
      content: Section("サビ".into()),
    },
    Token {
      line_num: 43,
      content: Lyrics("これから幾千 輝く光年".into()),
    },
    Token {
      line_num: 44,
      content: Yomigana(
        RomanStr::new("これからいくせんかがやくこうねん").unwrap(),
      ),
    },
    Token {
      line_num: 45,
      content: Comment,
    },
    Token {
      line_num: 46,
      content: Time(MinuteSecond::new().seconds(50.5)),
    },
    Token {
      line_num: 47,
      content: Lyrics("あなたの願いは破壊神".into()),
    },
    Token {
      line_num: 48,
      content: Yomigana(
        RomanStr::new("あなたのねがいははかいしん").unwrap(),
      ),
    },
    Token {
      line_num: 49,
      content: Comment,
    },
    Token {
      line_num: 50,
      content: Time(MinuteSecond::new().seconds(53.1)),
    },
    Token {
      line_num: 51,
      content: Lyrics("今まで遁走 隠した暴走".into()),
    },
    Token {
      line_num: 52,
      content: Yomigana(
        RomanStr::new("いままでとんそうかくしたぼうそう").unwrap(),
      ),
    },
    Token {
      line_num: 53,
      content: Comment,
    },
    Token {
      line_num: 54,
      content: Time(MinuteSecond::new().seconds(55.6)),
    },
    Token {
      line_num: 55,
      content: Lyrics("してきたのにね".into()),
    },
    Token {
      line_num: 56,
      content: Yomigana(RomanStr::new("してきたのにね").unwrap()),
    },
    Token {
      line_num: 57,
      content: Comment,
    },
    Token {
      line_num: 58,
      content: Time(MinuteSecond::new().seconds(58.4)),
    },
    Token {
      line_num: 59,
      content: Lyrics("これでも幾千 してきた我慢".into()),
    },
    Token {
      line_num: 60,
      content: Yomigana(
        RomanStr::new("これでもいくせんしてきたがまん").unwrap(),
      ),
    },
    Token {
      line_num: 61,
      content: Comment,
    },
    Token {
      line_num: 62,
      content: Comment,
    },
    Token {
      line_num: 63,
      content: Time(MinuteSecond::new().minutes(1).seconds(1.1)),
    },
    Token {
      line_num: 64,
      content: Lyrics("でもやっぱりためらうや".into()),
    },
    Token {
      line_num: 65,
      content: Yomigana(
        RomanStr::new("でもやっぱりためらうや").unwrap(),
      ),
    },
    Token {
      line_num: 66,
      content: Comment,
    },
    Token {
      line_num: 67,
      content: Time(MinuteSecond::new().minutes(1).seconds(3.9)),
    },
    Token {
      line_num: 68,
      content: Comment,
    },
    Token {
      line_num: 69,
      content: Time(MinuteSecond::new().minutes(1).seconds(4.7)),
    },
    Token {
      line_num: 70,
      content: Lyrics("もっといい未来はもう無い".into()),
    },
    Token {
      line_num: 71,
      content: Yomigana(
        RomanStr::new("もっといいみらいはもうない").unwrap(),
      ),
    },
    Token {
      line_num: 72,
      content: Comment,
    },
    Token {
      line_num: 73,
      content: Time(MinuteSecond::new().minutes(1).seconds(8.7)),
    },
    Token {
      line_num: 74,
      content: Caption("(間奏)".into()),
    },
    Token {
      line_num: 75,
      content: Comment,
    },
    Token {
      line_num: 76,
      content: Time(MinuteSecond::new().minutes(1).seconds(25.9)),
    },
    Token {
      line_num: 77,
      content: Section("二番".into()),
    },
    Token {
      line_num: 78,
      content: Comment,
    },
    Token {
      line_num: 79,
      content: Time(MinuteSecond::new().minutes(1).seconds(27.9)),
    },
    Token {
      line_num: 80,
      content: Section("Aメロ".into()),
    },
    Token {
      line_num: 81,
      content: Lyrics("内に惑う思い その原点に".into()),
    },
    Token {
      line_num: 82,
      content: Yomigana(
        RomanStr::new("うちにまどうおもいそのげんてんに").unwrap(),
      ),
    },
    Token {
      line_num: 83,
      content: Comment,
    },
    Token {
      line_num: 84,
      content: Time(MinuteSecond::new().minutes(1).seconds(32.7)),
    },
    Token {
      line_num: 85,
      content: Lyrics("触れ写しても 心は得られない".into()),
    },
    Token {
      line_num: 86,
      content: Yomigana(
        RomanStr::new("ふれうつしてもこころはえられない").unwrap(),
      ),
    },
    Token {
      line_num: 87,
      content: Comment,
    },
    Token {
      line_num: 88,
      content: Time(MinuteSecond::new().minutes(1).seconds(37.5)),
    },
    Token {
      line_num: 89,
      content: Lyrics("ひた走った海岸も 今では馬鹿らしくて".into()),
    },
    Token {
      line_num: 90,
      content: Yomigana(
        RomanStr::new("ひたはしったかいがんもいまではばからしくて")
          .unwrap(),
      ),
    },
    Token {
      line_num: 91,
      content: Comment,
    },
    Token {
      line_num: 92,
      content: Time(MinuteSecond::new().minutes(1).seconds(42.3)),
    },
    Token {
      line_num: 93,
      content: Lyrics(
        "透明なこのファージは 山となって積み上がる".into(),
      ),
    },
    Token {
      line_num: 94,
      content: Yomigana(
        RomanStr::new(
          "とうめいなこのふぁーじはやまとなってつみあがる",
        )
        .unwrap(),
      ),
    },
    Token {
      line_num: 95,
      content: Comment,
    },
    Token {
      line_num: 96,
      content: Time(MinuteSecond::new().minutes(1).seconds(46.9)),
    },
    Token {
      line_num: 97,
      content: Section("Bメロ".into()),
    },
    Token {
      line_num: 98,
      content: Lyrics(
        "唯一、力だけが無い この不思議な僕たちは".into(),
      ),
    },
    Token {
      line_num: 99,
      content: Yomigana(
        RomanStr::new(
          "ゆいいつちからだけがないこのふしぎなぼくたちは",
        )
        .unwrap(),
      ),
    },
    Token {
      line_num: 100,
      content: Comment,
    },
    Token {
      line_num: 101,
      content: Time(MinuteSecond::new().minutes(1).seconds(51.3)),
    },
    Token {
      line_num: 102,
      content: Lyrics("紡いだ回路だけで 不条理を壊せる".into()),
    },
    Token {
      line_num: 103,
      content: Yomigana(
        RomanStr::new("つむいだかいろだけでふじょうりをこわせる")
          .unwrap(),
      ),
    },
    Token {
      line_num: 104,
      content: Comment,
    },
    Token {
      line_num: 105,
      content: Time(MinuteSecond::new().minutes(1).seconds(55.6)),
    },
    Token {
      line_num: 106,
      content: Section("サビ".into()),
    },
    Token {
      line_num: 107,
      content: Lyrics("これまで幾千 交わした沿線".into()),
    },
    Token {
      line_num: 108,
      content: Yomigana(
        RomanStr::new("これまでいくせんかわしたえんせん").unwrap(),
      ),
    },
    Token {
      line_num: 109,
      content: Comment,
    },
    Token {
      line_num: 110,
      content: Time(MinuteSecond::new().minutes(1).seconds(58.0)),
    },
    Token {
      line_num: 111,
      content: Lyrics("あなたとは友じゃいられない".into()),
    },
    Token {
      line_num: 112,
      content: Yomigana(
        RomanStr::new("あなたとはともじゃいられない").unwrap(),
      ),
    },
    Token {
      line_num: 113,
      content: Comment,
    },
    Token {
      line_num: 114,
      content: Lyrics("|2".into()),
    },
    Token {
      line_num: 115,
      content: Time(MinuteSecond::new().minutes(2).seconds(0.4)),
    },
    Token {
      line_num: 116,
      content: Lyrics("今まで棒線 引いた凱旋".into()),
    },
    Token {
      line_num: 117,
      content: Yomigana(
        RomanStr::new("いままでぼうせんひいたがいせん").unwrap(),
      ),
    },
    Token {
      line_num: 118,
      content: Comment,
    },
    Token {
      line_num: 119,
      content: Time(MinuteSecond::new().minutes(2).seconds(2.9)),
    },
    Token {
      line_num: 120,
      content: Lyrics("してきたのにさ".into()),
    },
    Token {
      line_num: 121,
      content: Yomigana(RomanStr::new("してきたのにさ").unwrap()),
    },
    Token {
      line_num: 122,
      content: Comment,
    },
    Token {
      line_num: 123,
      content: Time(MinuteSecond::new().minutes(2).seconds(5.5)),
    },
    Token {
      line_num: 124,
      content: Lyrics("これでも幾千 してきた我慢".into()),
    },
    Token {
      line_num: 125,
      content: Yomigana(
        RomanStr::new("これでもいくせんしてきたがまん").unwrap(),
      ),
    },
    Token {
      line_num: 126,
      content: Comment,
    },
    Token {
      line_num: 127,
      content: Time(MinuteSecond::new().minutes(2).seconds(8.2)),
    },
    Token {
      line_num: 128,
      content: Lyrics("でもやっぱりためらうや".into()),
    },
    Token {
      line_num: 129,
      content: Yomigana(
        RomanStr::new("でもやっぱりためらうや").unwrap(),
      ),
    },
    Token {
      line_num: 130,
      content: Comment,
    },
    Token {
      line_num: 131,
      content: Time(MinuteSecond::new().minutes(2).seconds(10.8)),
    },
    Token {
      line_num: 132,
      content: Comment,
    },
    Token {
      line_num: 133,
      content: Time(MinuteSecond::new().minutes(2).seconds(11.5)),
    },
    Token {
      line_num: 134,
      content: Lyrics("嘘偽りの上でしか".into()),
    },
    Token {
      line_num: 135,
      content: Yomigana(
        RomanStr::new("うそいつわりのうえでしか").unwrap(),
      ),
    },
    Token {
      line_num: 136,
      content: Comment,
    },
    Token {
      line_num: 137,
      content: Time(MinuteSecond::new().minutes(2).seconds(15.4)),
    },
    Token {
      line_num: 138,
      content: Comment,
    },
    Token {
      line_num: 139,
      content: Time(MinuteSecond::new().minutes(2).seconds(24.3)),
    },
    Token {
      line_num: 140,
      content: Lyrics("生きられない".into()),
    },
    Token {
      line_num: 141,
      content: Yomigana(RomanStr::new("いきられない").unwrap()),
    },
    Token {
      line_num: 142,
      content: Comment,
    },
    Token {
      line_num: 143,
      content: Time(MinuteSecond::new().minutes(2).seconds(26.0)),
    },
    Token {
      line_num: 144,
      content: Comment,
    },
    Token {
      line_num: 145,
      content: Command("end".into()),
    },
    Token {
      line_num: 146,
      content: Comment,
    },
  ];

  let mut dur = Duration::new(0.0, 18.6).unwrap();
  let expected = Scoremap {
    metadata: {
      let mut m: HashMap<String, String> = HashMap::new();
      m.insert("title".into(), "満点星の約束".into());
      m.insert("song_author".into(), "Mikuro さいな".into());
      m.insert("singer".into(), "塩音ルト CERANA".into());
      m.insert("score_author".into(), "Mikuro さいな".into());
      m.insert("song_data".into(), "twinkle-grace.ogg".into());
      m.insert("bpm".into(), "200".into());
      ScoremapMetadata(m)
    },
    sections: Sections::new(vec![
      vec![Note::caption(dur.clone(), "満点星の約束")],
      vec![
        Note::sentence(
          dur.following_replace(4.9),
          Sentence::new(
            "体が浮くような 3000m の星空",
            "からだがうくようなさんぜんめーとるのほしぞら",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(5.0),
          Sentence::new(
            "初めて来た日に 交わした願いが光る",
            "はじめてきたひにかわしたねがいがひかる",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(4.8),
          Sentence::new(
            "遠くの街の光が 煌めきを隠して",
            "とおくのまちのひかりがきらめきをかくして",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(4.7),
          Sentence::new(
            "今夜も少しずつ 希望が融けてゆく",
            "こんやもすこしきぼうずつきぼうがとけてゆく",
          )
          .unwrap(),
        ),
      ],
      vec![
        Note::sentence(
          dur.following_replace(4.7),
          Sentence::new(
            "ああ僕ひとりの力じゃ 何にもできないだろうさ",
            "ああぼくひとりのちからじゃなんにもできないだろうさ",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(4.9),
          Sentence::new(
            "築いた繋がりなら 宇宙を越えられる",
            "きづいたつながりならうちゅうをこえられる",
          )
          .unwrap(),
        ),
      ],
      vec![
        Note::sentence(
          dur.following_replace(2.9),
          Sentence::new(
            "これから幾千 輝く光年",
            "これからいくせんかがやくこうねん",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(2.6),
          Sentence::new(
            "あなたの願いは破壊神",
            "あなたのねがいははかいしん",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(2.5),
          Sentence::new(
            "今まで遁走 隠した暴走",
            "いままでとんそうかくしたぼうそう",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(2.8),
          Sentence::new("してきたのにね", "してきたのにね").unwrap(),
        ),
        Note::sentence(
          dur.following_replace(2.7),
          Sentence::new(
            "これでも幾千 してきた我慢",
            "これでもいくせんしてきたがまん",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(2.8),
          Sentence::new(
            "でもやっぱりためらうや",
            "でもやっぱりためらうや",
          )
          .unwrap(),
        ),
        Note::blank(dur.following_replace(0.8)),
        Note::sentence(
          dur.following_replace(4.0),
          Sentence::new(
            "もっといい未来はもう無い",
            "もっといいみらいはもうない",
          )
          .unwrap(),
        ),
        Note::caption(dur.following_replace(17.2), "(間奏)"),
      ],
      vec![Note::blank(dur.following_replace(2.0))],
      vec![
        Note::sentence(
          dur.following_replace(4.8),
          Sentence::new(
            "内に惑う思い その原点に",
            "うちにまどうおもいそのげんてんに",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(4.8),
          Sentence::new(
            "触れ写しても 心は得られない",
            "ふれうつしてもこころはえられない",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(4.8),
          Sentence::new(
            "ひた走った海岸も 今では馬鹿らしくて",
            "ひたはしったかいがんもいまではばからしくて",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(4.6),
          Sentence::new(
            "透明なこのファージは 山となって積み上がる",
            "とうめいなこのふぁーじはやまとなってつみあがる",
          )
          .unwrap(),
        ),
      ],
      vec![
        Note::sentence(
          dur.following_replace(4.4),
          Sentence::new(
            "唯一、力だけが無い この不思議な僕たちは",
            "ゆいいつちからだけがないこのふしぎなぼくたちは",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(4.3),
          Sentence::new(
            "紡いだ回路だけで 不条理を壊せる",
            "つむいだかいろだけでふじょうりをこわせる",
          )
          .unwrap(),
        ),
      ],
      vec![
        Note::sentence(
          dur.following_replace(2.4),
          Sentence::new(
            "これまで幾千 交わした沿線",
            "これまでいくせんかわしたえんせん",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(2.4),
          Sentence::new(
            "あなたとは友じゃいられない",
            "あなたとはともじゃいられない",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(2.5),
          Sentence::new(
            "今まで棒線 引いた凱旋",
            "いままでぼうせんひいたがいせん",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(2.6),
          Sentence::new("してきたのにさ", "してきたのにさ").unwrap(),
        ),
        Note::sentence(
          dur.following_replace(2.7),
          Sentence::new(
            "これでも幾千 してきた我慢",
            "これでもいくせんしてきたがまん",
          )
          .unwrap(),
        ),
        Note::sentence(
          dur.following_replace(2.6),
          Sentence::new(
            "でもやっぱりためらうや",
            "でもやっぱりためらうや",
          )
          .unwrap(),
        ),
        Note::blank(dur.following_replace(0.7)),
        Note::sentence(
          dur.following_replace(3.9),
          Sentence::new(
            "嘘偽りの上でしか",
            "うそいつわりのうえでしか",
          )
          .unwrap(),
        ),
        Note::blank(dur.following_replace(8.9)),
        Note::sentence(
          dur.following_replace(1.7),
          Sentence::new("生きられない", "いきられない").unwrap(),
        ),
        Note::blank(dur.following_replace(1.0)),
      ],
    ]),
  };
  let actual = parse(input.as_slice())?;

  assert_eq!(expected.metadata.0, actual.metadata.0);

  for (section_idx, (expected, actual)) in expected
    .sections
    .iter()
    .zip(actual.sections.iter())
    .enumerate()
  {
    for (note_idx, (expected, actual)) in
      expected.iter().zip(actual.iter()).enumerate()
    {
      assert_eq!(
        expected.duration(),
        actual.duration(),
        "section_idx: {} | note_idx: {}",
        section_idx,
        note_idx
      );
      assert_eq!(
        expected.content(),
        actual.content(),
        "section_idx: {} | note_idx: {}",
        section_idx,
        note_idx
      );
    }
    assert_eq!(
      expected.len(),
      actual.len(),
      "section_idx: {}",
      section_idx
    );
  }

  Ok(())
}

#[test]
fn case2() -> Result<(), ScoremapParseError> {
  let input = vec![
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
      content: TokenContent::Time(MinuteSecond::new().seconds(4.0)),
    },
    Token {
      line_num: 15,
      content: TokenContent::Lyrics("えっ".into()),
    },
    Token {
      line_num: 16,
      content: TokenContent::Yomigana(RomanStr::new("えっ").unwrap()),
    },
    Token {
      line_num: 17,
      content: TokenContent::Time(MinuteSecond::new().seconds(6.0)),
    },
    Token {
      line_num: 18,
      content: TokenContent::Command("end".into()),
    },
  ];

  let mut duration = Duration::new(0.0, 2.22).unwrap();
  let expected = Scoremap {
    metadata: {
      let mut m: HashMap<String, String> = HashMap::new();
      m.insert("title".into(), "TEST".into());
      m.insert("score_author".into(), "Mikuro さいな".into());
      m.insert("song_data".into(), "void.ogg".into());
      m.insert("bpm".into(), "222.22".into());
      ScoremapMetadata(m)
    },
    sections: Sections::new(vec![vec![
      Note::blank(duration.clone()),
      Note::sentence(
        duration.following_replace(0.78),
        Sentence::new("打鍵テスト", "だけんてすと").unwrap(),
      ),
      Note::blank(duration.following_replace(1.0)),
      Note::sentence(
        duration.following_replace(2.0),
        Sentence::new("えっ", "えっ").unwrap(),
      ),
      Note::blank(duration.following_replace(1.0)),
    ]]),
  };
  let actual = parse(input.as_slice())?;

  assert_eq!(expected.metadata.0, actual.metadata.0);

  for (section_idx, (expected, actual)) in expected
    .sections
    .iter()
    .zip(actual.sections.iter())
    .enumerate()
  {
    for (note_idx, (expected, actual)) in
      expected.iter().zip(actual.iter()).enumerate()
    {
      assert_eq!(
        expected.content(),
        actual.content(),
        "section_idx: {} | note_idx: {}",
        section_idx,
        note_idx
      );
      assert_eq!(
        expected.duration(),
        actual.duration(),
        "section_idx: {} | note_idx: {}",
        section_idx,
        note_idx
      );
    }
    assert_eq!(
      expected.len(),
      actual.len(),
      "section_idx: {}",
      section_idx
    );
  }

  Ok(())
}
