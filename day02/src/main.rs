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

fn round_score_2(round: &str) -> i32 {
    let v: Vec<&str> = round.split(' ').collect();
    match v[..] {
        ["A", "X"] => LOOSE + SCISSORS,
        ["A", "Y"] => DRAW + ROCK,
        ["A", "Z"] => WIN + PAPER,
        ["B", "X"] => LOOSE + ROCK,
        ["B", "Y"] => DRAW + PAPER,
        ["B", "Z"] => WIN + SCISSORS,
        ["C", "X"] => LOOSE + PAPER,
        ["C", "Y"] => DRAW + SCISSORS,
        ["C", "Z"] => WIN + ROCK,
        _ => panic!("Invalid round"),
    }
}

fn main() {
    let file_text = fs::read_to_string("input").expect("Unable to read file");
    let rounds: Vec<&str> = file_text.lines().collect();
    let score_1: i32 = rounds.iter().map(|round| round_score(round)).sum();
    let score_2: i32 = rounds.iter().map(|round| round_score_2(round)).sum();
    println!("Score 1: {}", score_1);
    println!("Score 2: {}", score_2);
}
