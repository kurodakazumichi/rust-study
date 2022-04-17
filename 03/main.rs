#[derive(Debug)]
enum CHIP{
  Empty,
  Block,
  Player,
  Pien,
  Heart,
  HeartOnPien,
}

fn main(){
    // ステージデータを用意
    let stage_data = "
##########
#        #
# P   H G#
#     H  #
#        #
#H       #
#     G  #
#G       #
##########
";

  // 文字列→TILEの配列へ変換する
  let mut stage:Vec<CHIP> = Vec::new();

  for ch in stage_data.chars() 
  {
    match ch {
      '#' => { stage.push(CHIP::Block); }
      ' ' => { stage.push(CHIP::Empty); }
      'P' => { stage.push(CHIP::Player); }
      'G' => { stage.push(CHIP::Pien); }
      'H' => { stage.push(CHIP::Heart); }
      _   => {}
    };
  }
  
  loop {
    // ステージを描画する
    for (i, tile) in stage.iter().enumerate() 
    {
      let c = match *tile {
        CHIP::Player      => { '😆' }
        CHIP::Pien        => { '🥺' }
        CHIP::Heart       => { '💖' }
        CHIP::Block       => { '🟫' }
        CHIP::Empty       => { '⬜' }
        CHIP::HeartOnPien => { '🥰' }
      };

      if i % 10 == 0 {
        println!("");
      }
      print!("{}", c);
    }

    // クリア判定
    if let None = stage.iter().position(|chip| matches!(chip, CHIP::Pien)) {
      break;
    }

    // ユーザー入力を促すメッセージを表示
    println!("\nコマンド入力\n");
    println!("👆:w 👇:s 👈:a 👉:d\n");

    // ユーザーの入力を受け取る
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed read_line.");

    // 入力された文字列の最初の1文字を取得
    let direction = input.chars().next().expect("input error");

    // プレイヤーの現在位置を取得
    let current_pos = stage.iter().position(|chip| matches!(chip, CHIP::Player)).unwrap() as i32;

    // 移動先の座標を計算
    let next = match direction {
      'w' => { (current_pos - 10, current_pos - 20) }
      's' => { (current_pos + 10, current_pos + 20) }
      'a' => { (current_pos - 1 , current_pos - 2 ) }
      'd' => { (current_pos + 1 , current_pos + 2 ) }
      _   => { (current_pos     , current_pos) }
    };

    // 一歩先が床の場合移動
    if next.0 < 0 || stage.len() as i32 <= next.0 {
      continue;
    }

    if matches!(stage[next.0 as usize], CHIP::Empty) {
      stage.swap(current_pos as usize, next.0 as usize);
      continue;
    }

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
  }

  // クリアメッセージ
  println!("");
  println!("\n🥰CLEAR!! HAPPY!!🥰");
  println!("\n");
}
