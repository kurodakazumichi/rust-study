#[derive(Debug)]
enum TILE {
  Empty,
  Block,
  Player,
  Pien,
  Heart,
  HeartOnPien,
}

fn main() -> std::io::Result<()>{

  let stage_data = "
##########
#        #
# P   H G#
#     H  #
#        #
#H       #
#     G  #
#G       #
##########";

  let mut stage:Vec<TILE> = Vec::new();

  for ch in stage_data.chars() {

    match ch {
      '#' => { stage.push(TILE::Block); }
      ' ' => { stage.push(TILE::Empty); }
      'P' => { stage.push(TILE::Player); } 
      'G' => { stage.push(TILE::Pien); }
      'H' => { stage.push(TILE::Heart); }
      _ => {}
    };
  }

  loop {
    for (i, tile) in stage.iter().enumerate()
    {
      let c = match *tile {
        TILE::Player      => { '😆' }
        TILE::Pien        => { '🥺' }
        TILE::Heart       => { '💖' }
        TILE::Block       => { '🟫' }
        TILE::Empty       => { '⬜' }   
        TILE::HeartOnPien => { '🥰' }
      };
      
      if i % 10 == 0 {
        println!();
      }
      print!("{}", c);
    }

    if let None = stage.iter().position(|v| matches!(v, TILE::Pien)) {
      break;
    }

    println!("\nコマンド入力：\n");
    println!("👆:w 👇:s 👈:a 👉:d\n");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let direction = input.chars().next().unwrap();
    
    let player_index = stage.iter()
      .position(|v| matches!(v, TILE::Player))
      .unwrap() as i32;
    
    if direction == 'q' {
      break;
    }

    let next = match direction {
      'w' => { (player_index - 10, player_index - 20) }
      's' => { (player_index + 10, player_index + 20) }
      'a' => { (player_index - 1, player_index - 2) }
      'd' => { (player_index + 1, player_index + 2) }
      _ => { (player_index, player_index) }
    };

    // 範囲外は無視
    if next.0 < 0 || stage.len() <= next.0 as usize {
      continue;
    }

    // 移動先が何もなければ単純に移動
    if matches!(stage[next.0 as usize], TILE::Empty) {
      stage.swap(player_index as usize, next.0 as usize);
      continue;
    }

    // 移動先がハート
    if matches!(stage[next.0 as usize], TILE::Heart) 
    {
      // もう一歩先が範囲外
      if next.1 < 0 || 100 <= next.1 {
        continue;
      }

      // もう一歩先が空き地の場合
      if matches!(stage[next.1 as usize], TILE::Empty) 
      {
        stage[next.1 as usize] = TILE::Heart;
        stage[next.0 as usize] = TILE::Player;
        stage[player_index as usize] = TILE::Empty;
      }

      // もう一歩先がぴえんの場合
      if matches!(stage[next.1 as usize], TILE::Pien) 
      {
        stage[next.1 as usize] = TILE::HeartOnPien;
        stage[next.0 as usize] = TILE::Player;
        stage[player_index as usize] = TILE::Empty;
      }
    }
  }
  
  println!("");
  println!("\n🥰CLEAR!! HAPPY!!🥰");
  println!("\n");

  Ok(())
}
