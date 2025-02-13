mod piece;

use piece::{
    MainCalculate,
    check_move
};
use std::{
    io::{
        Write as io_Write,
        stdin,
        BufRead,
    },
};

static mut BOARD_X_SIZE: usize = 0;
static mut BOARD_Y_SIZE: usize = 0;

fn main() {
    let mut result = MainCalculate::default();
    result.calculate_move(0);
    let a = result.save_moves.as_can_moves().unwrap().1.keys().collect::<Vec<_>>();
    unsafe {
        BOARD_X_SIZE = 8;
        BOARD_Y_SIZE = 8;
    }

    let mut reader = stdin().lock().lines().map(|a| a.unwrap());
    println!("{:?}", a);
    println!("{:?}", check_move(a, reader.next().unwrap()));
}
