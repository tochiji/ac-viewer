# ac-viewer
AtCoderに提出されたコードをTerminalから閲覧するコマンドラインツールです。

https://user-images.githubusercontent.com/7910558/150673713-cf37731e-bacf-4c3c-9ae8-ba40a48eb000.mov


## インストール

デスクトップなど、適当なディレクトリで下記を実行します。

※ `cargo`コマンドがない場合は、[rustup を事前にインストール](https://www.rust-lang.org/ja/tools/install)してください。

```console
$ gh repo clone tochiji/ac-viewer
$ cargo install --path ac-viewer
```

下記のように表示されれば、インストール成功です。

```console
...
Installed package `ac_viewer v0.1.1 ....
```

## コマンドの実行

AtCoderのURLに対応したコンテストIDと問題IDを入力してください。

下記の例では、ABC150 D問題の過去提出を表示します。（RustのACコードのみ、ソート順はコード長の昇順）

```console
$ ac_viewer abc150 d

? Select a Submission (Ctrl+c to exit) › 
❯ 001  Rust (1.42.0)  2020-09-21 11:39:50(JST)    369 Byte   23 ms cunitac
  002  Rust (1.42.0)  2021-01-30 00:32:39(JST)    490 Byte   20 ms EEDECA
  003  Rust (1.42.0)  2021-08-09 20:36:54(JST)    506 Byte   32 ms boiler
  004  Rust (1.42.0)  2020-08-08 01:16:05(JST)    514 Byte   33 ms magurofly
  005  Rust (1.42.0)  2021-11-26 06:36:09(JST)    517 Byte   28 ms gregson
  006  Rust (1.42.0)  2021-07-08 00:14:12(JST)    528 Byte   31 ms ntk_ta01
  007  Rust (1.42.0)  2021-07-06 21:38:10(JST)    547 Byte   46 ms yaumu3
  008  Rust (1.42.0)  2021-09-01 08:18:02(JST)    550 Byte   34 ms boiler
  009  Rust (1.42.0)  2021-09-27 07:46:44(JST)    552 Byte   28 ms boiler
  010  Rust (1.42.0)  2021-03-24 20:43:41(JST)    563 Byte   37 ms kawashige
  011  Rust (1.42.0)  2021-10-12 08:11:53(JST)    563 Byte   28 ms boiler
  012  Rust (1.42.0)  2021-07-16 20:38:14(JST)    576 Byte   36 ms y_yu
  013  Rust (1.42.0)  2021-02-19 11:36:08(JST)    600 Byte   30 ms Raclett4
  014  Rust (1.42.0)  2021-06-07 21:20:21(JST)    604 Byte   36 ms bouzuya
(Move up and down to reveal more choices)
```

上下キーで提出を選び、エンターを押すとその提出コードを確認できます。

## コマンドの終了

`Ctrl+c` でプログラムを終了できます。
