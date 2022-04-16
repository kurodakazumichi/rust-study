# ぴえんにハートを届ける 2

## 概要

1. ユーザー入力を促すメッセージを表示
2. ユーザーの入力を取得する
3. 入力された文字列の最初の1文字を得る
4. 無限ループを作る

## 1. ユーザー入力を促すメッセージを表示

```rust
// プレイヤーに入力してもらうようにメッセージを表示
println!("\nコマンド入力：\n");
println!("👆:w 👇:s 👈:a 👉:d\n");
```



## 2.ユーザーの入力を受け取る

```rust
// ユーザーの入力を取得
let mut input = String::new();
let result = std::io::stdin().read_line(&mut input);

match result {
    Ok(n) => { println!("{}", n); }
    Err(e) => { println!("{:?}", e); }
}
```

`read_line`は失敗する可能性もあるので`Result型`を返すため、エラーハンドリング処理を書かないと警告がでる。

これは以下のように書き直す事もできる👇



```rust
// ユーザーの入力を取得
let mut input = String::new();
std::io::stdin().read_line(&mut input).expect("failed read_line.");
```

`expect`はエラーがあると指定されたメッセージを表示してプログラムを止める、これでちょっと楽に書ける。



### `input`の中身を確認

```rust
for c in input.chars() {
    println!("{}", c as u32);
}
```



## 3.入力された文字列の最初の1文字を得る

```rust
// 入力の1文字目を取得
let direction = input.chars().next().expect("input error.");
```



### 上記がエラーになるケース？

文字列が0文字だったら1文字目はないのでエラーになるはず

```rust
let s = String::from("");
s.chars().next().expect("error");
```



## 4.無限ループを作る

ゲームの基本の流れは「ステージを表示→ユーザ入力→ステージを表示→ユーザー入力」のように表示と入力を繰り返すので、今まで作った処理を無限ループするようにする。

```rust
loop {
    // 今まで作った処理
}
```



`loop`は`while(true)`と同じで無限ループになる。



## 次回

ユーザ入力の内容を判定してプレイヤーを移動させる。
