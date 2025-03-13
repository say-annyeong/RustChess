mod piece;

use std::{
    io::{
        Write as io_Write,
        stdin,
        BufRead,
    },
};
use piece::{
    MainCalculate,
    check_move,
    Board2D
};
use crate::piece::BoardXD;

static mut BOARD_X_SIZE: usize = 0;
static mut BOARD_Y_SIZE: usize = 0;

fn main() {
    let board = Board2D::default();
    println!("{}", board)
}
