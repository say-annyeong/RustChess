mod piece;

use std::{
    io::{
        Write as io_Write,
        BufRead,
    },
};
use piece::{
    Board2D
};

static mut BOARD_X_SIZE: usize = 0;
static mut BOARD_Y_SIZE: usize = 0;

fn main() {
    let board = Board2D::default();
    println!("{}", board)
}
