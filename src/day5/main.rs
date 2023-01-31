#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]

use std::fs;
use itertools::Itertools;
use crate::crate_mover::{CrateMover, CrateMoverVersion};
use crate::stack::Stack;

mod mv;
mod stack;
mod crate_mover;

fn main() {
    let input =
        fs::read_to_string("inputs/day5")
            .expect("Day 5 input file should exist");

    let stacks = stack::parse_stacks(&input);
    let moves = mv::parse_moves(&input);

    // Part 1
    let crate_mover_9000 = CrateMover {
        version: CrateMoverVersion::Version9000
    };
    let mut stacks_part1 = stacks.to_vec();
    crate_mover_9000.execute_program(&mut stacks_part1, &moves);
    let part1_answer = crates_on_top(&stacks_part1);

    println!("Part 1 answer: {}", part1_answer);

    // Part 2
    let crate_mover_9001 = CrateMover {
        version: CrateMoverVersion::Version9001
    };
    let mut stacks_part2 = stacks.to_vec();
    crate_mover_9001.execute_program(&mut stacks_part2, &moves);
    let part2_answer = crates_on_top(&stacks_part2);

    println!("Part 2 answer: {}", part2_answer);
}

fn crates_on_top(stacks: &Vec<Stack>) -> String {
    (&stacks)
        .into_iter()
        .map(|stack| stack.items.last().unwrap().to_owned())
        .join("")
        .to_owned()
}