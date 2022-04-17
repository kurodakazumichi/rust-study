# ぴえんにハートを届ける 3

- 入力された方向にプレイヤーを移動させる。
- クリア判定



## プレイヤーの移動とは？

- ステージは1次元配列なので、位置は配列のIndexで表せる。
- 移動は配列の要素を交換したり、値を変更することで実装する。
- 右に移動する
  - Indexを+1
- 左に移動する
  - Indexを-1
- 下に移動する
  - Indexを+10
- 上に移動する
  - Indexを-10



## 仕様

- 入力された方向にプレイヤーを移動させる。
- プレイヤーは壁、またぴえんがいる場所には移動できない。
- 移動先が床(Empty)の場合
  - 移動する
- 移動先がハート(Heart)の場合
  - ハートの先が床(Empty)の場合
    - プレイヤーとハートを移動する
  - ハートの先がぴえんの場合
    - ぴえんを幸せにし、ハートの位置にプレイヤーが移動する
  - ハートの先が上記以外
    - 移動できない



## 概要

1. プレイヤーの現在位置を取得
2. 移動先の位置を計算
3. 一歩先が床の場合の移動
4. 一歩先がハートの場合の移動
5. クリア判定



## 1. プレイヤーの現在位置を取得

- プレイヤーの現在地＝`stage`の中の`CHIP::Player`のIndex
- enumの比較は`matches!`マクロを使う

```rust
let current_pos = stage.iter().position(|chip| matches!(chip, CHIP::Player)).unwrap() as i32;
```





## 2.移動先の位置を計算

- 移動先は一歩先、二歩先の2か所を計算で求めておく。
- タプルを使う。

```rust
// 移動先の座標を計算
let next = match direction {
    'w' => { (current_pos - 10, current_pos - 20) }
    's' => { (current_pos + 10, current_pos + 20) }
    'a' => { (current_pos - 1 , current_pos - 2 ) }
    'd' => { (current_pos + 1 , current_pos + 2 ) }
    _   => { (current_pos     , current_pos) }
};
```



### usizeのoverflow問題

- usize型の変数と引き算をした結果がマイナスになるとエラーになる

```rust
let a:usize = 0;
let b = a - 1; // エラー
```



## 3.一歩先が床の場合の移動

- 一歩先の座標がベクタの範囲外であれば移動しない(できない)
- ベクタの添え字は`usize`なので、判定や指定時には`usize`にキャストしている

```rust
// 一歩先が床の場合移動
if next.0 < 0 || stage.len() as i32 <= next.0 {
    continue;
}

if matches!(stage[next.0 as usize], CHIP::Empty) {
    stage.swap(current_pos as usize, next.0 as usize);
    continue;
}
```



## 4. 一歩先がハートの場合の移動

- 二歩先の座標がベクタの範囲外であれば移動しない(できない)
- 現在位置、一歩先、二歩先がどうなるかを考えてベクタの値を更新する

```rust
// 一歩先がハートの場合
if matches!(stage[next.0 as usize], CHIP::Heart) 
{
    // 二歩先が範囲外かチェック
    if next.1 < 0 || stage.len() as i32 <= next.1 {
        continue;
    }

    // 二歩先が床の場合
    if matches!(stage[next.1 as usize], CHIP::Empty) {
        stage[next.1 as usize] = CHIP::Heart;
        stage[next.0 as usize] = CHIP::Player;
        stage[current_pos as usize] = CHIP::Empty;
    }

    // 二歩先がぴえんの場合
    if matches!(stage[next.1 as usize], CHIP::Pien) {
        stage[next.1 as usize] = CHIP::HeartOnPien;
        stage[next.0 as usize] = CHIP::Player;
        stage[current_pos as usize] = CHIP::Empty;
    }
}
```



## 5. クリア判定

- クリア＝`stage`の中にぴえんがいない
- クリアしてたら無限ループを抜ける
- クリアメッセージを表示



クリア判定をどのタイミングで行うか？

​	👉ステージを表示した直後に行う

```rust
// クリア判定
if let None = stage.iter().position(|chip| matches!(chip, CHIP::Pien)) {
    break;
}
```



```rust
// クリアメッセージ
println!("");
println!("\n🥰CLEAR!! HAPPY!!🥰");
println!("\n");
```



