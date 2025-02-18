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

static mut BOARD_X_SIZE: usize = 0;
static mut BOARD_Y_SIZE: usize = 0;

fn main() {
    unsafe {
        BOARD_X_SIZE = 8;
        BOARD_Y_SIZE = 8;
    }

    let mut result = MainCalculate::default();
    result.calculate_move(0);
    let a = result.save_moves.as_can_moves().unwrap().1.keys().collect::<Vec<_>>();

    let board = Board2D::new(Vec::from([unsafe { BOARD_X_SIZE }, unsafe { BOARD_Y_SIZE }]));

    let mut reader = stdin().lock().lines().map(|a| a.unwrap());
    println!("{:?}", a);
    println!("{:?}", check_move(a, reader.next().unwrap()));
}
