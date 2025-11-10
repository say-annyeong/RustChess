use crate::code::Code;
use crate::movement::State;

pub struct Piece {
    piece_name: String,
    piece_short_name: String,
    piece_score: i32,
    piece_state: State,
    piece_code: Code,
}