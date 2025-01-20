mod piece;

use piece::{
    MainCalculate,
    MoveType
};
use std::{
    io::{
        Write as io_Write,
        stdin,
        BufRead,
    },
};
use std::ops::Index;
use regex::Regex;

static mut BOARD_X_SIZE: usize = 0;
static mut BOARD_Y_SIZE: usize = 0;

fn chess_x_convent(input: String) -> usize {
    let input_parse = input.trim().parse::<usize>().unwrap();
    let index: Vec<usize> = unsafe { (0..BOARD_X_SIZE).rev().collect() };
    index[input_parse - 1]
}

fn chess_y_convent(input: String) -> usize {
    input.chars().enumerate().map(|(radix, num)| (num.to_digit(36).unwrap() - 9) as usize * 26_usize.pow(radix as u32)).sum::<usize>() - 1
}

fn check_move(moves: Vec<&MoveType>, player_move: String) -> MoveType {
    let player_input_re = Regex::new(r"(?P<name>[A-Za-z]*)(?P<start_col>[A-Za-z]*)(?P<start_row>\d*)(?P<takes>[Xx]?)(?P<end_col>[A-Za-z]+)(?P<end_row>\d+)(?P<other>.*)").unwrap();
    if let Some(input) = player_input_re.captures(player_move.as_str()) {
        let (mut name, start_col, start_row, takes, end_col, end_row, other) = (input["name"].to_lowercase(), input["start_col"].to_lowercase(), input["start_row"].to_string(), if input["takes"].is_empty() { false } else { true }, input["end_col"].to_lowercase(), input["end_row"].to_string(), input["other"].to_lowercase());
        let cx = if start_col.is_empty() { None } else { Some(chess_y_convent(start_col)) };
        let cy = if start_row.is_empty() { None } else { Some(chess_x_convent(start_row)) };
        let x = chess_x_convent(end_row);
        let y = chess_y_convent(end_col);

        if name.is_empty() {
            name = "pawn".to_string();
        }

        let mut can_moves = Vec::new();
        for move_type in moves {
            let name_correct = Some(name.clone()) == move_type.piece_type;
            let start_col_correct = cx == move_type.cx || cx.is_none();
            let start_row_correct = cy == move_type.cy || cy.is_none();
            let end_col_correct = Some(x) == move_type.x;
            let end_row_correct = Some(y) == move_type.y;
            if name_correct && start_col_correct && start_row_correct && end_col_correct && end_row_correct {
                can_moves.push(move_type);
            }
        }

        if can_moves.len() == 1 {
            can_moves[0].clone()
        } else {
            MoveType::none()
        }
    } else {
        MoveType::none()
    }
}

fn main() {
    let mut result = MainCalculate::default();
    result.calculate_moves(1);
    let a = result.save_moves.as_can_moves().unwrap().1.keys().collect::<Vec<_>>();
    unsafe {
        BOARD_X_SIZE = 8;
        BOARD_Y_SIZE = 8;
    }

    let mut reader = stdin().lock().lines().map(|a| a.unwrap());
    println!("{:?}", a);
    println!("{:?}", check_move(a, reader.next().unwrap()));
}
