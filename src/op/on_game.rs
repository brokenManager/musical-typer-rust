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
      for typed in controller.key_press().iter() {
        self.activity.input(*typed);
      }

      let delta_time = controller.elapse_time();
      self.accumulated_time += delta_time;
      self.activity.update_time(self.accumulated_time);

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
    expected: Vec<PresentLog>,
    index: usize,
  }

  impl MockPresenter {
    fn new(expected: Vec<PresentLog>) -> Self {
      MockPresenter { expected, index: 0 }
    }

    fn log(&mut self, log: PresentLog) {
      assert_eq!(
        self.expected[self.index], log,
        "index: {}",
        self.index
      );
      self.index += 1;
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
      KeyPress(4.5, "tuyogaribakarijatukarerune"),
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
    let mut presenter = MockPresenter::new(vec![
      PlayBGM("kkiminochikara-edited.wav".to_owned()),
      DecreaseRemainingTime(3.0),
      UpdateSentence(Sentence::new(
        "もうダメだ そんな時は",
        "もうだめだそんなときは",
      )?),
      DecreaseRemainingTime(3.5),
      UpdateSentence(Sentence::new(
        "あの空を 見上げてごらん",
        "あのそらをみあげてごらん",
      )?),
      DecreaseRemainingTime(4.5),
      UpdateSentence(Sentence::new(
        "夜の闇を 押し上げて",
        "よるのやみをおしあげて",
      )?),
      DecreaseRemainingTime(3.5),
      UpdateSentence(Sentence::new(
        "太陽がまた 微笑みくれる",
        "たいようがまたほほえみくれる",
      )?),
      DecreaseRemainingTime(4.5),
      UpdateSentence(Sentence::new(
        "前だけ見てても疲れるね",
        "まえだけみててもつかれるね",
      )?),
      DecreaseRemainingTime(3.5),
      UpdateSentence(Sentence::new(
        "たまには立ち止まっていいんだよ",
        "たまにはたちどまっていいんだよ",
      )?),
      DecreaseRemainingTime(4.5),
      UpdateSentence(Sentence::new(
        "胸いっぱい空気吸ったら",
        "むねいっぱいくうきすったら",
      )?),
      DecreaseRemainingTime(3.75),
      UpdateSentence(Sentence::new(
        "また足を踏み出そうよ",
        "またあしをふみだそうよ",
      )?),
      DecreaseRemainingTime(4.25),
      UpdateSentence(Sentence::new(
        "僕らの夢 空を超えて",
        "ぼくらのゆめそらをこえて",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "果てなく広がって行くよ",
        "はてなくひろがっていくよ",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "涙のあとも 胸の痛みも",
        "なみだのあともむねのいたみも",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "キミの力になる",
        "きみのちからになる",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "真っ暗だ！ 投げ出す前に",
        "まっくらだなげだすまえに",
      )?),
      DecreaseRemainingTime(3.0),
      UpdateSentence(Sentence::new(
        "あの空を 見上げてごらん",
        "あのそらをみあげてごらん",
      )?),
      DecreaseRemainingTime(5.0),
      UpdateSentence(Sentence::new(
        "夜の闇 目を凝らせば",
        "よるのやみめをこらせば",
      )?),
      DecreaseRemainingTime(3.5),
      UpdateSentence(Sentence::new(
        "星たちのダンスパーティー",
        "ほしたちのだんすぱーてぃー",
      )?),
      DecreaseRemainingTime(4.5),
      UpdateSentence(Sentence::new(
        "強がりばかりじゃ疲れるね",
        "つよがりばかりじゃつかれるね",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "涙を零してもいいんだよ",
        "なみだをこぼしてもいいんだよ",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "思い切り泣いたあとには",
        "おもいきりないたあとには",
      )?),
      DecreaseRemainingTime(3.75),
      UpdateSentence(Sentence::new(
        "また笑顔をみせてね",
        "またえがおをみせてね",
      )?),
      DecreaseRemainingTime(4.25),
      UpdateSentence(Sentence::new(
        "みんなの夢 時を超えて",
        "みんなのゆめときをこえて",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "どこまでも繋がって行くよ",
        "どこまでもつながっていくよ",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "涙のあとも 胸の痛みも",
        "なみだのあともむねのいたみも",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "キミの力になる",
        "きみのちからになる",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "少しづつ 前に進もう",
        "すこしづつまえにすすもう",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "未来はずっと待っているから",
        "みらいはずっとまっているから",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "思い切り泣いた夜さえ",
        "おもいきりないたよるさえ",
      )?),
      DecreaseRemainingTime(3.5),
      UpdateSentence(Sentence::new(
        "いつか思い出に変わるよ",
        "いつかおもいでにかわるよ",
      )?),
      DecreaseRemainingTime(6.5),
      UpdateSentence(Sentence::new(
        "僕らの夢 空を超えて",
        "ぼくらのゆめそらをこえて",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "果てなく広がって行くよ",
        "はてなくひろがっていくよ",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "涙のあとも 胸の痛みも",
        "なみだのあともむねのいたみも",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "キミの力になる",
        "きみのちからになる",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "みんなの夢 時を超えて",
        "みんなのゆめときをこえて",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "どこまでも繋がって行くよ",
        "どこまでもつながっていくよ",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "涙のあとも 胸の痛みも",
        "なみだのあともむねのいたみも",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "キミの力になる",
        "きみのちからになる",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "涙のあとも 胸の痛みも",
        "なみだのあともむねのいたみも",
      )?),
      DecreaseRemainingTime(4.0),
      UpdateSentence(Sentence::new(
        "キミの力になる",
        "きみのちからになる",
      )?),
      DecreaseRemainingTime(5.0),
    ]);

    game.run_game(&mut controller, &mut presenter)?;

    Ok(())
  }
}
