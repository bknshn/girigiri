#[macro_use]
extern crate lazy_static;

use std::time::Instant;

mod engine;
mod board;
mod csa;
// use engine::first_engine::*;
// use engine::random_engine::*;
use engine::alpha_beta_engine::*;
use board::state::*;
use board::hash::*;
use std::io::{self, Write};

const LOOP_MAX: i32 = 2000;
const NORMALIZE_TURNS: i32 = 100;

fn main() {
    let mut engine = AlphaBetaEngine::new(10, false);
    let mut counter = 0;
    let start = Instant::now();
    while counter < LOOP_MAX {
        engine.state = State::new();
        unsafe {
            HASH_TABLE = [HASH_ENTRY_NONE; HASH_TABLE_SIZE];
        }
        engine.randomize_state();
        if engine.proceed_move_learn() {
            if counter % NORMALIZE_TURNS + 1 == NORMALIZE_TURNS {
                engine.evaluator.normalize(NORMALIZE_TURNS);
            }
            counter += 1;
            print!("\r{} / {} finished", counter + 1, LOOP_MAX);
            io::stdout().flush().unwrap();
        }
    }
    let elapsed = start.elapsed();
    print!(
        "\rLearning finished. Total time: {}.{:03} sec\n",
        elapsed.as_secs(),
        elapsed.subsec_nanos() / 1_000_000
    );
    engine.evaluator.save_pps();
    engine.evaluator.save_ppo();
}
