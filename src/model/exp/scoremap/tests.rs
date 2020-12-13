#[test]
fn case1() -> Result<(), super::ScoremapError> {
  use super::{super::time::Duration, lexer::ScoremapLoadConfig};
  use crate::model::exp::scoremap::sections::section::note::Note;

  let score = super::Scoremap::from_str(
    r#"
# Case 1
:title 満点星の約束
:song_author Mikuro さいな
:singer 塩音ルト CERANA
:score_author Mikuro さいな
:song_data twinkle-grace.ogg
:bpm 200

[start]
*0.0
@一番
>> 満点星の約束

*18.6
@Aメロ
体が浮くような 3000m の星空
:からだがうくようなさんぜんめーとるのほしぞら

*23.5
初めて来た日に 交わした願いが光る
:はじめてきたひにかわしたねがいがひかる

*28.5
遠くの街の光が 煌めきを隠して
:とおくのまちのひかりがきらめきをかくして

*33.3
今夜も少しずつ 希望が融けてゆく
:こんやもすこしきぼうずつきぼうがとけてゆく

*38.0
@Bメロ
ああ僕ひとりの力じゃ 何にもできないだろうさ
:ああぼくひとりのちからじゃなんにもできないだろうさ

*42.7
築いた繋がりなら 宇宙を越えられる
:きづいたつながりならうちゅうをこえられる

*47.6
@サビ
これから幾千 輝く光年
:これからいくせんかがやくこうねん

*50.5
あなたの願いは破壊神
:あなたのねがいははかいしん

*53.1
今まで遁走 隠した暴走
:いままでとんそうかくしたぼうそう

*55.6
してきたのにね
:してきたのにね

*58.4
これでも幾千 してきた我慢
:これでもいくせんしてきたがまん

|1
*1.1
でもやっぱりためらうや
:でもやっぱりためらうや

*3.9

*4.7
もっといい未来はもう無い
:もっといいみらいはもうない

*8.7
>> (間奏)

*25.9
@ 二番

*27.9
@Aメロ
内に惑う思い その原点に
:うちにまどうおもいそのげんてんに

*32.7
触れ写しても 心は得られない
:ふれうつしてもこころはえられない

*37.5
ひた走った海岸も 今では馬鹿らしくて
:ひたはしったかいがんもいまではばからしくて

*42.3
透明なこのファージは 山となって積み上がる
:とうめいなこのふぁーじはやまとなってつみあがる

*46.9
@Bメロ
唯一、力だけが無い この不思議な僕たちは
:ゆいいつちからだけがないこのふしぎなぼくたちは

*51.3
紡いだ回路だけで 不条理を壊せる
:つむいだかいろだけでふじょうりをこわせる

*55.6
@サビ
これまで幾千 交わした沿線
:これまでいくせんかわしたえんせん

*58.0
あなたとは友じゃいられない
:あなたとはともじゃいられない

|2
*0.4
今まで棒線 引いた凱旋
:いままでぼうせんひいたがいせん

*2.9
してきたのにさ
:してきたのにさ

*5.5
これでも幾千 してきた我慢
:これでもいくせんしてきたがまん

*8.2
でもやっぱりためらうや
:でもやっぱりためらうや

*10.8

*11.5
嘘偽りの上でしか
:うそいつわりのうえでしか

*15.4

*24.3
生きられない
:いきられない

*26.0

[end]
  "#,
    |config: ScoremapLoadConfig| -> ScoremapLoadConfig { config },
  )?;
  use super::sections::section::note::sentence::Sentence;
  let mut dur = Duration::new(0.0, 18.6).unwrap();
  let expected_notes: Vec<Vec<Note>> = vec![
    vec![Note::caption(dur.clone(), "満点星の約束")],
    vec![
      Note::sentence(
        dur.following_replace(4.9),
        Sentence::new(
          "体が浮くような 3000m の星空",
          "からだがうくようなさんぜんめーとるのほしぞら",
        )?,
      ),
      Note::sentence(
        dur.following_replace(5.0),
        Sentence::new(
          "初めて来た日に 交わした願いが光る",
          "はじめてきたひにかわしたねがいがひかる",
        )?,
      ),
      Note::sentence(
        dur.following_replace(4.8),
        Sentence::new(
          "遠くの街の光が 煌めきを隠して",
          "とおくのまちのひかりがきらめきをかくして",
        )?,
      ),
      Note::sentence(
        dur.following_replace(4.7),
        Sentence::new(
          "今夜も少しずつ 希望が融けてゆく",
          "こんやもすこしきぼうずつきぼうがとけてゆく",
        )?,
      ),
    ],
    vec![
      Note::sentence(
        dur.following_replace(4.7),
        Sentence::new(
          "ああ僕ひとりの力じゃ 何にもできないだろうさ",
          "ああぼくひとりのちからじゃなんにもできないだろうさ",
        )?,
      ),
      Note::sentence(
        dur.following_replace(4.9),
        Sentence::new(
          "築いた繋がりなら 宇宙を越えられる",
          "きづいたつながりならうちゅうをこえられる",
        )?,
      ),
    ],
    vec![
      Note::sentence(
        dur.following_replace(2.9),
        Sentence::new(
          "これから幾千 輝く光年",
          "これからいくせんかがやくこうねん",
        )?,
      ),
      Note::sentence(
        dur.following_replace(2.6),
        Sentence::new(
          "あなたの願いは破壊神",
          "あなたのねがいははかいしん",
        )?,
      ),
      Note::sentence(
        dur.following_replace(2.5),
        Sentence::new(
          "今まで遁走 隠した暴走",
          "いままでとんそうかくしたぼうそう",
        )?,
      ),
      Note::sentence(
        dur.following_replace(2.8),
        Sentence::new("してきたのにね", "してきたのにね")?,
      ),
      Note::sentence(
        dur.following_replace(2.7),
        Sentence::new(
          "これでも幾千 してきた我慢",
          "これでもいくせんしてきたがまん",
        )?,
      ),
      Note::sentence(
        dur.following_replace(2.8),
        Sentence::new(
          "でもやっぱりためらうや",
          "でもやっぱりためらうや",
        )?,
      ),
      Note::blank(dur.following_replace(0.8)),
      Note::sentence(
        dur.following_replace(4.0),
        Sentence::new(
          "もっといい未来はもう無い",
          "もっといいみらいはもうない",
        )?,
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
        )?,
      ),
      Note::sentence(
        dur.following_replace(4.8),
        Sentence::new(
          "触れ写しても 心は得られない",
          "ふれうつしてもこころはえられない",
        )?,
      ),
      Note::sentence(
        dur.following_replace(4.8),
        Sentence::new(
          "ひた走った海岸も 今では馬鹿らしくて",
          "ひたはしったかいがんもいまではばからしくて",
        )?,
      ),
      Note::sentence(
        dur.following_replace(4.6),
        Sentence::new(
          "透明なこのファージは 山となって積み上がる",
          "とうめいなこのふぁーじはやまとなってつみあがる",
        )?,
      ),
    ],
    vec![
      Note::sentence(
        dur.following_replace(4.4),
        Sentence::new(
          "唯一、力だけが無い この不思議な僕たちは",
          "ゆいいつちからだけがないこのふしぎなぼくたちは",
        )?,
      ),
      Note::sentence(
        dur.following_replace(4.3),
        Sentence::new(
          "紡いだ回路だけで 不条理を壊せる",
          "つむいだかいろだけでふじょうりをこわせる",
        )?,
      ),
    ],
    vec![
      Note::sentence(
        dur.following_replace(2.4),
        Sentence::new(
          "これまで幾千 交わした沿線",
          "これまでいくせんかわしたえんせん",
        )?,
      ),
      Note::sentence(
        dur.following_replace(2.4),
        Sentence::new(
          "あなたとは友じゃいられない",
          "あなたとはともじゃいられない",
        )?,
      ),
      Note::sentence(
        dur.following_replace(2.5),
        Sentence::new(
          "今まで棒線 引いた凱旋",
          "いままでぼうせんひいたがいせん",
        )?,
      ),
      Note::sentence(
        dur.following_replace(2.6),
        Sentence::new("してきたのにさ", "してきたのにさ")?,
      ),
      Note::sentence(
        dur.following_replace(2.7),
        Sentence::new(
          "これでも幾千 してきた我慢",
          "これでもいくせんしてきたがまん",
        )?,
      ),
      Note::sentence(
        dur.following_replace(2.6),
        Sentence::new(
          "でもやっぱりためらうや",
          "でもやっぱりためらうや",
        )?,
      ),
      Note::blank(dur.following_replace(0.7)),
      Note::sentence(
        dur.following_replace(3.9),
        Sentence::new(
          "嘘偽りの上でしか",
          "うそいつわりのうえでしか",
        )?,
      ),
      Note::blank(dur.following_replace(8.9)),
      Note::sentence(
        dur.following_replace(1.7),
        Sentence::new("生きられない", "いきられない")?,
      ),
      Note::blank(dur.following_replace(1.0)),
    ],
  ];

  for (section_index, (expected, actual)) in
    expected_notes.iter().zip(score.sections.iter()).enumerate()
  {
    for (note_index, (expected, actual)) in
      expected.iter().zip(actual.iter()).enumerate()
    {
      assert_eq!(
        *expected.content(),
        *actual.content(),
        "section_index: {}, note_index: {}",
        section_index,
        note_index
      );
      assert_eq!(
        expected.duration(),
        actual.duration(),
        "section_index: {}, note_index: {}",
        section_index,
        note_index
      );
    }
    assert_eq!(
      expected.len(),
      actual.len(),
      "section_index: {}",
      section_index
    );
  }
  assert_eq!(expected_notes.len(), score.sections.len());
  for (k, v) in score.metadata.0.iter() {
    assert_eq!(
      v,
      match k.as_str() {
        "title" => "満点星の約束",
        "song_author" => "Mikuro さいな",
        "singer" => "塩音ルト CERANA",
        "score_author" => "Mikuro さいな",
        "song_data" => "twinkle-grace.ogg",
        "bpm" => "200",
        _ => unreachable!(),
      }
    );
  }
  Ok(())
}

#[test]
fn case2() -> Result<(), super::ScoremapError> {
  use super::sections::section::note::Note;
  use crate::model::exp::time::Duration;
  let score = super::Scoremap::from_str(
    r#"
# Sample 1
:title TEST
:score_author Mikuro さいな
:song_data void.ogg
:bpm 222.22

[start]
*2.22
打鍵テスト
:だけんてすと

*3.0
[end]
"#,
    |config| config.ignore_unsupported_property(true),
  )?;
  use super::sections::section::note::sentence::Sentence;
  let mut dur = Duration::new(0.0, 2.22).unwrap();
  let expected_notes: Vec<Vec<Note>> = vec![vec![
    Note::blank(dur.clone()),
    Note::sentence(
      dur.following_replace(0.78),
      Sentence::new("打鍵テスト", "だけんてすと")?,
    ),
    Note::blank(dur.following_replace(1.0)),
  ]];

  for (section_index, (expected, actual)) in
    expected_notes.iter().zip(score.sections.iter()).enumerate()
  {
    for (note_index, (expected, actual)) in
      expected.iter().zip(actual.iter()).enumerate()
    {
      assert_eq!(
        *expected.content(),
        *actual.content(),
        "section_index: {}, note_index: {}",
        section_index,
        note_index
      );
      assert_eq!(
        expected.duration(),
        actual.duration(),
        "section_index: {}, note_index: {}",
        section_index,
        note_index
      );
    }
    assert_eq!(
      expected.len(),
      actual.len(),
      "section_index: {}",
      section_index
    );
  }
  assert_eq!(expected_notes.len(), score.sections.len());

  Ok(())
}
