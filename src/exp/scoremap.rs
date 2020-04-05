use std::collections::HashMap;

use super::note::Note;

pub mod lexer;
pub mod parser;

use lexer::{ScoremapLexError, ScoremapLoadConfig};
use parser::ScoremapParseError;

#[derive(Debug)]
pub enum ScoremapError {
  LexError(ScoremapLexError),
  ParseError(ScoremapParseError),
}

pub type ScoremapMetadata = HashMap<String, String>;

#[derive(Debug)]
pub struct Scoremap {
  pub metadata: ScoremapMetadata,
  pub notes: Vec<Note>,
}

impl Scoremap {
  pub fn from_str<C>(
    string: &str,
    configurator: C,
  ) -> Result<Self, ScoremapError>
  where
    C: FnOnce(ScoremapLoadConfig) -> ScoremapLoadConfig,
  {
    use ScoremapError::*;

    use std::io::BufReader;
    let reader = BufReader::new(string.as_bytes());
    let tokens = lexer::lex(
      configurator(lexer::ScoremapLoadConfig::new()),
      reader,
    )
    .map_err(|e| LexError(e))?;

    Ok(parser::parse(tokens.into_iter()).map_err(|e| ParseError(e))?)
  }

  pub fn from_file<C>(
    file: std::fs::File,
    configurator: C,
  ) -> Result<Self, ScoremapError>
  where
    C: FnOnce(ScoremapLoadConfig) -> ScoremapLoadConfig,
  {
    use ScoremapError::*;

    use std::io::BufReader;
    let reader = BufReader::new(file);
    let tokens = lexer::lex(
      configurator(lexer::ScoremapLoadConfig::new()),
      reader,
    )
    .map_err(|e| LexError(e))?;

    Ok(parser::parse(tokens.into_iter()).map_err(|e| ParseError(e))?)
  }
}

#[test]
fn case1() {
  let score = Scoremap::from_str(
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
    |config: lexer::ScoremapLoadConfig| -> lexer::ScoremapLoadConfig {
      config
    },
  )
  .unwrap();
  println!("{:#?}", score);
  assert!(loop {
    break true;
  })
}
