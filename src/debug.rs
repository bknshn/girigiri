#[macro_use]
extern crate lazy_static;

use std::time::Instant;
use std::io;
use std::io::Read;
use std::str::FromStr;

mod board;
use board::state::*;
use board::move_encode::*;

fn main() {
    let mut state = State::new();
    let mut mv = NULL_MOVE;
    loop {
        state.print_debug();
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("failed to read line");
        let bytes = input.as_bytes();
        if bytes.len() == 2 {
            // undo
            state.print_move(&mv);
            state.undo_move(&mv);
        } else {
            let to_j = '9' as i8 - bytes[2] as i8;
            let to_i = bytes[3] as i8 - 'a' as i8;
            if bytes[1] == '*' as u8 {
                // drop
                let kind = match bytes[0] as char {
                    'K' => 7,
                    'G' => 6,
                    'R' => 5,
                    'B' => 4,
                    'S' => 3,
                    'N' => 2,
                    'L' => 1,
                    'P' => 0,
                    _ => 7,
                };
                mv = board::move_encode::Move::drop_encode(kind, to_i, to_j);
            } else {
                let from_j = '9' as i8 - bytes[0] as i8;
                let from_i = bytes[1] as i8 - 'a' as i8;
                if bytes.len() == 6 {
                    // promote
                    mv = board::move_encode::Move::promote_encode(from_i, from_j, to_i, to_j);
                } else if bytes.len() == 5 {
                    // normal
                    mv = board::move_encode::Move::normal_encode(from_i, from_j, to_i, to_j);
                }
            }
            state.print_move(&mv);
            state.apply_move(&mv);
        }
    }
}
