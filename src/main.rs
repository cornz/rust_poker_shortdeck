#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use serde::Deserialize;


#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Calculation {
    player1: Option<String>,
    player2: Option<String>,
    player3: Option<String>,
    player4: Option<String>,
    player5: Option<String>,
    player6: Option<String>,
    board: Option<String>
}

#[post("/", format = "json", data = "<calculation>")]
fn calc(calculation: Json<Calculation>) -> String {
    let mut ranges2 = vec![];
    let mut board:u64 = 0;
    if !calculation.player1.is_none() {
        ranges2.push(calculation.player1.as_ref().unwrap())
    }
    if !calculation.player2.is_none() {
        ranges2.push(calculation.player2.as_ref().unwrap())
    }
    if !calculation.player3.is_none() {
        ranges2.push(calculation.player3.as_ref().unwrap())
    }
    if !calculation.player4.is_none() {
        ranges2.push(calculation.player4.as_ref().unwrap())
    }
    if !calculation.player5.is_none() {
        ranges2.push(calculation.player5.as_ref().unwrap())
    }
    if !calculation.player6.is_none() {
        ranges2.push(calculation.player6.as_ref().unwrap())
    }
    if !calculation.board.is_none() {
        board = get_card_mask(&*calculation.board.as_ref().unwrap())
    }


    use std::time::Instant;
    let before = Instant::now();
    use rust_poker::hand_range::{HandRange, get_card_mask};
    use rust_poker::equity_calculator::calc_equity;
    //let ranges = HandRange::from_strings(["TJs,T9s,QJs".to_string(), "AQ+".to_string(), "AK".to_string()].to_vec());
    let poker_ranges = HandRange::from_strings_unwrapped(ranges2);
    //let public_cards = get_card_mask(&"6c7c8h".to_string());
    let mut n_games = 10000;
    let n_threads = 4;
    let mut equities = calc_equity(&*poker_ranges, board, n_threads, n_games);
    println!("{:?}", equities);
    println!("Iterations: {:?}", n_games);
    println!("Elapsed time: {:.2?}", before.elapsed());
    format!("{:?}",equities)
}

fn main() {
    rocket::ignite()
        .mount("/calc", routes![calc])
        .launch();
}
