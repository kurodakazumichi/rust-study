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
}
