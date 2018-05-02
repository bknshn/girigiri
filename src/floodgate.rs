#[macro_use]
extern crate lazy_static;

mod csa;
mod engine;
mod board;
use csa::player::*;
use std::{thread, time};

const USERNAME: &str = "girigiri";
const PASSWORD: &str = "floodgate-300-10F,hoge";

fn main() {
    let mut player = CsaPlayer::new(("wdoor.c.u-tokyo.ac.jp", 4081));
    player.login(USERNAME, PASSWORD);
    println!("waiting for a game...");
    player.find_game_auto();
    println!("\n{}\n", player.client.read());
    player.init_turn();

    player.play();
}