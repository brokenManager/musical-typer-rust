# Musical Typer Rust

Musical Typer を Rust で実装チャレンジ!!!


# 仕様

楽曲が流れながら、歌詞とそのローマ字が画面に表示されます。

制限時間内に素早くタイピングして、譜面をクリアしましょう。


## イベント

画面=>ゲーム:
- キー入力
- 時間の経過

ゲーム=>画面:
- BGM の再生/停止
- 残り時間の減少
- 入力する文字列の更新


## 画面

- タイトル
- 作者
- 今のセクション
  - 残り時間
  - 入力する歌詞
    - すでに入力した文字列
    - これから入力する文字列
  - 入力するローマ字
    - すでに入力した文字列
    - これから入力する文字列
- 仮想キーボード


## 譜面

- メタデータ
  - 曲名
  - 作者
  - 歌手
  - 譜面作者
  - 楽曲ファイルパス
- コマンド
  - BPM
  - セクション
    - 時間
    - 分(省略可)
    - 歌詞
    - 歌詞の読み仮名
