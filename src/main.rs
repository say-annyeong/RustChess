mod piece;

use std::io::{stdin, BufRead};
use crate::piece::{MainCalculate2D, ParsePlayerInput2D};

fn main() {
    let mut play_board = MainCalculate2D::default();
    println!("{}", play_board.board);
    let read = stdin().lock().lines();
    let mut reader = read.map(|line| line.unwrap());
    let input = reader.next().unwrap();
    let possible_move = play_board.calculate_moved(2);
    let moves: Vec<_> = possible_move.as_can_moves().unwrap().1.keys().cloned().collect();
    let parse_player_input = ParsePlayerInput2D::new(moves);
    let parse_input = parse_player_input.parse_player_input(input);
    println!("parse input: {:?}", parse_input)
}
