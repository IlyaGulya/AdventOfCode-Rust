extern crate core;

use std::fs;
use outcome::Outcome;
use mv::Move;
use round::Round;
use strategy::Strategy;

mod outcome;
mod mv;
mod strategy;
mod round;
mod result;

fn main() {
    let input =
        fs::read_to_string("inputs/day2")
            .expect("Day 2 input file should exist");

    let strategies = parse_strategies(input);
    let rounds: Vec<Round> = create_rounds(strategies);
    let score: i32 = count_score(rounds);

    println!("{:?}", score)
}

fn parse_strategies(input: String) -> Vec<Strategy> {
    input
        .lines()
        .map(|line| {
            let (opponent_move, my_strategy) =
                line.split_once(" ")
                    .expect("Only two items per row is allowed!");

            let opponent_move = Move::by_str(opponent_move);
            let my_outcome = result::Result::by_str(my_strategy);

            Strategy {
                opponent_move,
                my_outcome,
            }
        })
        .collect()
}

fn create_rounds(strategies: Vec<Strategy>) -> Vec<Round> {
    strategies
        .into_iter()
        .map(|Strategy { opponent_move, my_outcome }| {
            Round {
                opponent_move,
                my_move: match my_outcome {
                    result::Result::LOSE => Move::losing_move(opponent_move),
                    result::Result::DRAW => opponent_move,
                    result::Result::WIN => Move::winning_move(opponent_move),
                },
            }
        })
        .collect()
}

fn count_score(rounds: Vec<Round>) -> i32 {
    rounds
        .into_iter()
        .map(Outcome::of_round)
        .map(|outcome| {
            let result_score = match outcome.result {
                result::Result::WIN => 6,
                result::Result::DRAW => 3,
                result::Result::LOSE => 0,
            };

            outcome.move_score + result_score
        })
        .sum()
}
