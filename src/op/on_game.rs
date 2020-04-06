use crate::exp::game_activity::GameActivity;
use crate::exp::minute_second::Seconds;
use crate::exp::note::Section;
use crate::exp::roman::roman_lexer::RomanParseError;
use crate::exp::scoremap::lexer::ScoremapLexError;
use crate::exp::scoremap::{Scoremap, ScoremapError};
use crate::exp::sentence::Sentence;

pub trait Controller {
  fn key_press(&mut self) -> Vec<char>;
  fn elapse_time(&mut self) -> f64;
}
pub trait Presenter {
  fn play_bgm(&mut self, name: &str);
  fn decrease_remaining_time(&mut self, delta_time: f64);
  fn update_sentence(&mut self, string: &Sentence);
  fn mistyped(&mut self);
  fn flush_screen(&mut self);
}

#[derive(Debug)]
pub enum MusicalTyperError {
  SongDataNotFound,
  FileReadError { reason: String },
  ScoremapBuildError(ScoremapError),
}

impl From<std::io::Error> for MusicalTyperError {
  fn from(err: std::io::Error) -> Self {
    MusicalTyperError::FileReadError {
      reason: err.to_string(),
    }
  }
}

impl From<ScoremapError> for MusicalTyperError {
  fn from(err: ScoremapError) -> Self {
    MusicalTyperError::ScoremapBuildError(err)
  }
}
impl From<RomanParseError> for MusicalTyperError {
  fn from(_err: RomanParseError) -> Self {
    MusicalTyperError::ScoremapBuildError(ScoremapError::LexError(
      ScoremapLexError::InvalidStatementDefinition {
        line_num: 1,
        reason: "ふりがなでのそのような平仮名の並びは未対応です。",
      },
    ))
  }
}

pub struct MusicalTyper {
  score: Scoremap,
  activity: GameActivity,
  accumulated_time: Seconds,
}

impl MusicalTyper {
  pub fn new(score: Scoremap) -> Self {
    let activity = GameActivity::new(&score.notes);
    MusicalTyper {
      score,
      activity,
      accumulated_time: 0.0,
    }
  }

  pub fn run_game(
    &mut self,
    controller: &mut impl Controller,
    presenter: &mut impl Presenter,
  ) -> Result<(), MusicalTyperError> {
    use MusicalTyperError::*;

    let metadata = &self.score.metadata;
    if let Some(ref song_data) = metadata.get("song_data") {
      presenter.play_bgm(song_data);
    } else {
      return Err(SongDataNotFound);
    }

    self.activity.update_time(0.0);
    while let Some(Section {
      foreign_note,
      from,
      to,
    }) = self.activity.current_section()
    {
      let delta_time = controller.elapse_time();
      self.accumulated_time += delta_time;
      self.activity.update_time(self.accumulated_time);

      for typed in controller.key_press().iter() {
        self.activity.input(*typed);
      }

      presenter.decrease_remaining_time(delta_time);
      if let Some(sentence) = self.activity.current_sentence() {
        presenter.update_sentence(sentence);
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::{Controller, MusicalTyperError, Presenter};
  use crate::exp::sentence::Sentence;

  struct KeyPress(f64, &'static str);

  struct MockController {
    key_press_schedule: &'static [KeyPress],
  }

  impl MockController {
    fn new(key_press_schedule: &'static [KeyPress]) -> Self {
      MockController { key_press_schedule }
    }
  }

  impl Controller for MockController {
    fn key_press(&mut self) -> Vec<char> {
      let res = self.key_press_schedule[0].1.chars().collect();
      self.key_press_schedule = &self.key_press_schedule[1..];
      res
    }
    fn elapse_time(&mut self) -> f64 {
      self.key_press_schedule[0].0
    }
  }

  #[derive(Debug, PartialEq)]
  enum PresentLog {
    PlayBGM(String),
    DecreaseRemainingTime(f64),
    UpdateSentence(Sentence),
    Mistyped,
  }

  use PresentLog::*;

  struct MockPresenter {
    log: Vec<PresentLog>,
  }

  impl MockPresenter {
    fn new() -> Self {
      MockPresenter { log: vec![] }
    }

    fn log(&mut self, log: PresentLog) {
      println!("{:#?}", log);
      self.log.push(log);
    }

    fn logs(&self) -> &[PresentLog] {
      &self.log
    }
  }

  impl Presenter for MockPresenter {
    fn play_bgm(&mut self, name: &str) {
      self.log(PlayBGM(name.to_owned()));
    }
    fn decrease_remaining_time(&mut self, delta_time: f64) {
      self.log(DecreaseRemainingTime(delta_time));
    }
    fn update_sentence(&mut self, string: &Sentence) {
      self.log(UpdateSentence(string.clone()));
    }
    fn mistyped(&mut self) {
      self.log(Mistyped)
    }
    fn flush_screen(&mut self) {}
  }

  #[test]
  fn op1() -> Result<(), MusicalTyperError> {
    use crate::exp::scoremap::Scoremap;
    use crate::op::on_game::MusicalTyper;

    let test_score = Scoremap::from_file(
      std::fs::File::open(std::path::Path::new(
        "example/sampleScore.tsc",
      ))?,
      |config| config.ignore_invalid_properties(true),
    )?;

    let mut game = MusicalTyper::new(test_score);

    let mut controller = MockController::new(&[
      KeyPress(3.0, ""),
      KeyPress(3.0, "moudamedasonnnatokiha"),
      KeyPress(3.5, "anosorawomiagetegorann"),
      KeyPress(4.5, "yorunoyamiwoosiagete"),
      KeyPress(3.5, "taiyougamatahohoemikureru"),
      KeyPress(4.5, "maedakemitetemortukarerune"),
      KeyPress(3.5, "tamanihatatidomatteiinndayo"),
      KeyPress(4.5, "muneippaikuukisuttara"),
      KeyPress(3.75, "mataashiwohumidasouyo"),
      KeyPress(4.25, "bokuranoyumesorawokoete"),
      KeyPress(4.0, "hateshinakuhirogatteikuyo"),
      KeyPress(4.0, "namidanoatomomunenoitamimo"),
      KeyPress(4.0, "kiminochikaraninaru"),
      KeyPress(4.0, "maltukuradanagedasumaeni"),
      KeyPress(3.0, "anosorawomiagetegorann"),
      KeyPress(5.0, "yorunoyamimewokoraseba"),
      KeyPress(3.5, "hoshitachinodannsupa-texi-"),
      KeyPress(3.5, "tuyogaribakarijatukarerune"),
      KeyPress(4.0, "namidawokoboshitemoiinndayo"),
      KeyPress(4.0, "omoikirinaitaatoniha"),
      KeyPress(3.75, "mataegaowomisetene"),
      KeyPress(4.25, "minnnanoyumetokiwokoete"),
      KeyPress(4.0, "dokomademotunagaxtuteikuyo"),
      KeyPress(4.0, "namidanoatomomunenoitamimo"),
      KeyPress(4.0, "kiminochikaraninaru"),
      KeyPress(4.0, "sukoshidutumaenisusumou"),
      KeyPress(4.0, "miraihazuttomatteirukara"),
      KeyPress(4.0, "omoikirinaitayorusase"),
      KeyPress(3.5, "itukaomoidenikawaruyo"),
      KeyPress(6.5, "bokuranoyumesorawokoete"),
      KeyPress(4.0, "hateshinakuhirogatteikuyo"),
      KeyPress(4.0, "namidanoatomomunenoitamimo"),
      KeyPress(4.0, "kiminochikaraninaru"),
      KeyPress(4.0, "minnnanoyumetokiwokoete"),
      KeyPress(4.0, "dokomademotunagatteikuyo"),
      KeyPress(4.0, "namidanoatomomunenoitamimo"),
      KeyPress(4.0, "kiminochikaraninaru"),
      KeyPress(4.0, "namidanoatomomunenoitamimo"),
      KeyPress(4.0, "kiminochikaraninaru"),
      KeyPress(5.0, ""),
    ]);
    let mut presenter = MockPresenter::new();

    game.run_game(&mut controller, &mut presenter)?;

    let expected_log = &[
      PlayBGM("kkiminochikara-edited.wav".to_owned()),
      DecreaseRemainingTime(3.0),
      DecreaseRemainingTime(3.0),
      UpdateSentence(Sentence::new(
        "もうダメだ そんな時は",
        "もうだめだそんなときは",
      )?),
      /*
            あの空を 見上げてごらん
      :あのそらをみあげてごらん

      *11.000
      夜の闇を 押し上げて
      :よるのやみをおしあげて

      *14.500
      太陽がまた 微笑みくれる
      :たいようがまたほほえみくれる

      *19.000
      前だけ見てても疲れるね
      :まえだけみててもつかれるね

      *22.500
      たまには立ち止まっていいんだよ
      :たまにはたちどまっていいんだよ

      *27.000
      胸いっぱい空気吸ったら
      :むねいっぱいくうきすったら

      *30.750
      また足を踏み出そうよ
      :またあしをふみだそうよ

      *35.000
      僕らの夢 空を超えて
      :ぼくらのゆめそらをこえて

      *39.000
      果てなく広がって行くよ
      :はてなくひろがっていくよ

      *43.000
      涙のあとも 胸の痛みも
      :なみだのあともむねのいたみも

      *47.000
      キミの力になる
      :きみのちからになる

      *51.000
      真っ暗だ！ 投げ出す前に
      :まっくらだなげだすまえに

      *54.000
      あの空を 見上げてごらん
      :あのそらをみあげてごらん

      *59.000
      夜の闇 目を凝らせば
      :よるのやみめをこらせば


      |1
      *02.500
      星たちのダンスパーティー
      :ほしたちのだんすぱーてぃー

      *07.000
      強がりばかりじゃ疲れるね
      :つよがりばかりじゃつかれるね

      *11.000
      涙を零してもいいんだよ
      :なみだをこぼしてもいいんだよ

      *15.000
      思い切り泣いたあとには
      :おもいきりないたあとには

      *18.750
      また笑顔をみせてね
      :またえがおをみせてね

      *23.000
      みんなの夢 時を超えて
      :みんなのゆめときをこえて

      *27.000
      どこまでも繋がって行くよ
      :どこまでもつながっていくよ

      *31.0000
      涙のあとも 胸の痛みも
      :なみだのあともむねのいたみも

      *35.000
      キミの力になる
      :きみのちからになる

      *39.000
      少しづつ 前に進もう
      :すこしづつまえにすすもう

      *43.000
      未来はずっと待っているから
      :みらいはずっとまっているから

      *47.000
      思い切り泣いた夜さえ
      :おもいきりないたよるさえ

      *50.500
      いつか思い出に変わるよ
      :いつかおもいでにかわるよ

      *57.000
      僕らの夢 空を超えて
      :ぼくらのゆめそらをこえて

      |2
      *01.000
      果てなく広がって行くよ
      :はてなくひろがっていくよ

      *05.000
      涙のあとも 胸の痛みも
      :なみだのあともむねのいたみも

      *09.000
      キミの力になる
      :きみのちからになる

      *13.000
      みんなの夢 時を超えて
      :みんなのゆめときをこえて

      *17.000
      どこまでも繋がって行くよ
      :どこまでもつながっていくよ

      *21.000
      涙のあとも 胸の痛みも
      :なみだのあともむねのいたみも

      *25.000
      キミの力になる
      :きみのちからになる

      *29.000
      涙のあとも 胸の痛みも
      :なみだのあともむねのいたみも

      *33.000
      キミの力になる
      :きみのちからになる
      */
    ];
    for (expected, actual) in
      expected_log.iter().zip(presenter.logs().iter())
    {
      assert_eq!(expected, actual);
    }
    assert_eq!(expected_log.len(), presenter.logs().len());

    Ok(())
  }
}
