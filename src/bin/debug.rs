extern crate lazy_static;
extern crate girigiri;

use std::io;

use girigiri::shogi::state::State;
use girigiri::shogi::move_encode::*;

fn main() {
    let mut state = State::new();
    let mut mv = NULL_MOVE;
    loop {
        println!("{:?}", state);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        if input == "undo\n" {
            // undo
            state.undo_move(&mv);
        } else {
            mv = Move::from_usi(&input);
            state.print_move(&mv);
            state.apply_move(&mv);
        }
    }
}
