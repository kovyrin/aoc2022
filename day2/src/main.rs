use std::str::{Lines, Split};
use std::fs::read_to_string;

const INPUT_FILE: &str = "real-input.txt";

fn normalize_move(player_move: &str) -> u32 {
  match player_move {
    "A" | "X" => 1,
    "B" | "Y" => 2,
    "C" | "Z" => 3,
    _ => panic!("Invalid move: {}", player_move),
  }
}

fn main() {
  let input: String = read_to_string(INPUT_FILE).unwrap();
  let lines: Lines = input.lines();

  let calculate_outcome = |player1_move: u32, player2_move: u32| -> u32 {
    match (player1_move, player2_move) {
      // player 1 wins
      (1, 3) | (2, 1) | (3, 2) => 0,
      // player 2 wins
      (1, 2) | (2, 3) | (3, 1) => 6,
      // tie
      (1, 1) | (2, 2) | (3, 3) => 3,
      _ => panic!("Invalid moves: {} {}", player1_move, player2_move),
    }

  };

  let mut total_score: u32 = 0;
  for line in lines {
    if line.is_empty() {
      continue;
    }

    let mut moves: Split<&str> = line.split(" ");
    let player1_move: u32 = normalize_move(moves.next().unwrap());
    let player2_move: u32 = normalize_move(moves.next().unwrap());

    total_score += player2_move;
    let outcome: u32 = calculate_outcome(player1_move, player2_move);
    println!("{line} => {}", player2_move + outcome);

    total_score += outcome
  }

  println!("Total score: {}", total_score);
}
