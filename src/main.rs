mod piece;

use std::io::{stdin, BufRead, Write};
use std::fs::File;
use crate::piece::{MainCalculate2D, ParsePlayerInput2D};

fn main() {
    let mut play_board = MainCalculate2D::default();
    println!("{}", play_board.board);
    let read = stdin().lock().lines();
    let mut reader = read.map(|line| line.unwrap());
    play_board.calculate_move(0);
    let (_, buffer) = play_board.save_moves.as_can_moves().unwrap().clone();
    let moves: Vec<_> = buffer.keys().cloned().collect();
    let mut file = File::create("C:\\Users\\User\\Desktop\\coding\\Rust\\RustChess\\name.txt").unwrap();
    file.write(format!("{:?}", moves).as_bytes()).expect("TODO: panic message");
    println!("{:?}", moves);
    let parse_player_input = ParsePlayerInput2D::new(moves);
    let input = reader.next().unwrap();
    let parse_input = parse_player_input.parse_player_input(input);
    println!("parse input: {:?}", parse_input);
}
