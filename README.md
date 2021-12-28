# TetrisWithRust
RustとIcedでTetrisを作る。

## 操作方法

A, D: 横移動

S: ソフトドロップ

W: ハードドロップ

J, K: 回転

L: ホールド


スーパーローテーションシステム: 実装済

ネクスト表示: 実装済

![tetris](https://user-images.githubusercontent.com/63438515/134529925-9f04f9af-06ed-4fc4-94a7-81bc0edfaa92.gif)

## How to build
```
$ git clone https://github.com/lemolatoon/TetrisWithRust.git
$ cd TetrisWithRust
$ cargo run
```
*注意: 実行にはRustの実行環境が必要です。
```
# Unix系の場合
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
コンパイル時に次のpackageが必要な場合があります。
```
$ sudo apt install gcc pkg-config openssl libasound2-dev cmake build-essential python3 libfreetype6-dev libexpat1-dev libxcb-composite0-dev libssl-dev libx11-dev libfontconfig1-dev libxkbcommon-dev
```
