use std::str::{Lines, Split};
use std::fs::read_to_string;

const INPUT_FILE: &str = "real-input.txt";

#[derive(Clone, Copy)]
enum Move {
  ROCK = 1,
  PAPER = 2,
  SCISSORS = 3,
}

fn parse_move(player_move: &str) -> Move {
  match player_move {
    "A" => Move::ROCK,
    "B" => Move::PAPER,
    "C" => Move::SCISSORS,
    _ => panic!("Invalid move: {}", player_move),
  }
}

// 0 = player 1 wins
// 3 = tie
// 6 = player 2 wins
fn calculate_outcome(player1_move: Move, player2_move: Move) -> u32 {
  match (player1_move, player2_move) {
    (Move::ROCK, Move::ROCK) => 3,
    (Move::PAPER, Move::PAPER) => 3,
    (Move::SCISSORS, Move::SCISSORS) => 3,

    (Move::ROCK, Move::PAPER) => 6,
    (Move::PAPER, Move::SCISSORS) => 6,
    (Move::SCISSORS, Move::ROCK) => 6,

    (Move::ROCK, Move::SCISSORS) => 0,
    (Move::PAPER, Move::ROCK) => 0,
    (Move::SCISSORS, Move::PAPER) => 0,
  }
}

fn pick_player2_move(player1_move: Move, expected_outcome: &str) -> Move {
  match (player1_move, expected_outcome) {
    // expected_outcome = X => Player 2 loses
    (Move::ROCK, "X") => Move::SCISSORS,
    (Move::PAPER, "X") => Move::ROCK,
    (Move::SCISSORS, "X") => Move::PAPER,

    // expected_outcome = Y => Player 2 ties
    (Move::ROCK, "Y") => Move::ROCK,
    (Move::PAPER, "Y") => Move::PAPER,
    (Move::SCISSORS, "Y") => Move::SCISSORS,

    // expected_outcome = Z => Player 2 wins
    (Move::ROCK, "Z") => Move::PAPER,
    (Move::PAPER, "Z") => Move::SCISSORS,
    (Move::SCISSORS, "Z") => Move::ROCK,

    _ => panic!("Invalid expected outcome: {}", expected_outcome),
  }
}

fn main() {
  let input: String = read_to_string(INPUT_FILE).unwrap();
  let lines: Lines = input.lines();

  let mut total_score: u32 = 0;
  for line in lines {
    let mut moves: Split<&str> = line.split(" ");

    let player1_move = parse_move(moves.next().unwrap());
    let expected_outcome = moves.next().unwrap();
    let player2_move = pick_player2_move(player1_move, expected_outcome);

    let move_score: u32 = player2_move as u32;
    let outcome_score: u32 = calculate_outcome(player1_move, player2_move);

    total_score += move_score + outcome_score;
  }

  println!("Total score: {}", total_score);
}
