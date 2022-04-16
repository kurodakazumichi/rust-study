#[derive(Debug)]
enum CHIP {
  Empty,
  Block,
  Player,
  Pien,
  Heart,
  HeartOnPien,
}

fn main() 
{
  // ステージデータを準備する
  let stage_data = "
##########
#        #
#  P  H G#
#     H  #
#        #
#        #
#H    G  #
#G       #
##########
";

  // ステージデータをCHIPの配列に変換する
  let mut stage:Vec<CHIP> = Vec::new();
  
  for ch in stage_data.chars() 
  {
    match ch {
      '#' => { stage.push(CHIP::Block);  }
      ' ' => { stage.push(CHIP::Empty); }
      'P' => { stage.push(CHIP::Player); }
      'G' => { stage.push(CHIP::Pien); }
      'H' => { stage.push(CHIP::Heart); }
      _   => { }
    }
  }

  // 無限ループ
  loop 
  {
    // ステージをターミナルに表示(絵文字で)
    for (i, chip) in stage.iter().enumerate() 
    {
      let c = match *chip {
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

    // プレイヤーに入力してもらうようにメッセージを表示
    println!("\nコマンド入力：\n");
    println!("👆:w 👇:s 👈:a 👉:d\n");

    // ユーザーの入力を取得
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed read_line.");

    // 入力の1文字目を取得
    let direction = input.chars().next().expect("input error.");
  }
}
