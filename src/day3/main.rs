extern crate core;

use std::collections::HashSet;
use std::fs;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
struct Rucksack {
    full: String,
    first_compartment: String,
    second_compartment: String,
}

fn char_to_priority(char: char) -> u32 {
    match char {
        'a'..='z' => (char as u32) - 96,
        'A'..='Z' => (char as u32) - 38,
        _ => panic!("Unsupported priority: {}", char)
    }
}

fn main() {
    let input =
        fs::read_to_string("inputs/day3")
            .expect("Day 3 input file should exist");

    let rucksacks: Vec<Rucksack> =
        input
            .lines()
            .map(|line| {
                let middle = line.len() / 2;
                let (first_compartment, second_compartment) = line.split_at(middle);

                if first_compartment.len() != second_compartment.len() {
                    panic!("Parts are not equal!")
                }

                Rucksack {
                    full: line.to_string(),
                    first_compartment: first_compartment.to_string(),
                    second_compartment: second_compartment.to_string(),
                }
            })
            .collect();

    let rucksacks = Rc::new(rucksacks);

    let sum_of_priorities: u32 =
        rucksacks
            .deref()
            .into_iter()
            .map(|rucksack| {
                let first: HashSet<char> = HashSet::from_iter(rucksack.first_compartment.chars().into_iter());
                let second: HashSet<char> = HashSet::from_iter(rucksack.second_compartment.chars().into_iter());

                let intersection: Vec<char> = (&first & &second).into_iter().collect();

                if intersection.len() > 1 {
                    panic!("Two halfs of rucksack must have exactly one item in common! Got: {:?}", intersection)
                }

                let char =
                    intersection
                        .first()
                        .expect("Intersection between two halfs of rucksack has to have at least one item!");


                char_to_priority(*char)
            })
            .sum();

    println!("Part 1 answer: {}", sum_of_priorities);

    let groups: Vec<Vec<Rucksack>> =
        rucksacks
            .deref()
            .chunks(3)
            .map(|group| group.to_vec())
            .collect();

    let groups_badges: Vec<char> =
        groups
            .into_iter()
            .map(|group| {
                group
                    .into_iter()
                    .map(|rucksack| HashSet::from_iter(rucksack.full.chars()))
                    .reduce(|acc: HashSet<char>, item: HashSet<char>| {
                        &acc & &item
                    })
                    .expect("Group of elves is required to have at least three elves")
                    .into_iter()
                    .next()
                    .expect("Group of elves required to have a badge.")
            })
            .collect();

    let groups_priorities: Vec<u32> =
        groups_badges
            .into_iter()
            .map(|badge| char_to_priority(badge))
            .collect();

    let sum_of_groups_priorities: u32 =
        groups_priorities
            .into_iter()
            .sum();

    println!("Part 2 answer: {}", sum_of_groups_priorities)
}