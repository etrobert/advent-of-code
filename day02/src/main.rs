use std::fs;

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOOSE: i32 = 0;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn round_score(round: &str) -> i32 {
    let v: Vec<&str> = round.split(' ').collect();
    match v[..] {
        ["A", "X"] => DRAW + ROCK,
        ["A", "Y"] => WIN + PAPER,
        ["A", "Z"] => LOOSE + SCISSORS,
        ["B", "X"] => LOOSE + ROCK,
        ["B", "Y"] => DRAW + PAPER,
        ["B", "Z"] => WIN + SCISSORS,
        ["C", "X"] => WIN + ROCK,
        ["C", "Y"] => LOOSE + PAPER,
        ["C", "Z"] => DRAW + SCISSORS,
        _ => panic!("Invalid round"),
    }
}

fn main() {
    let file_text = fs::read_to_string("input").expect("Unable to read file");
    let score: i32 = file_text.lines().map(round_score).sum();
    println!("{}", score)
}
