use std::fs;
use crate::range::Range;

mod range;

struct SchedulePair {
    first: Range,
    second: Range,
}

fn main() {
    let input =
        fs::read_to_string("inputs/day4")
            .expect("Day 3 input file should exist");

    let schedules: Vec<SchedulePair> =
        input
            .lines()
            .map(|line| {
                let (first_range, second_range) =
                    line
                        .split_once(",")
                        .expect(format!("Wrong input! Expected pair of ranges, got this instead: {}", line).as_str());

                SchedulePair {
                    first: Range::from_range_str(first_range),
                    second: Range::from_range_str(second_range),
                }
            })
            .collect();

    let fully_included_pairs_of_schedules_count =
        (&schedules)
            .into_iter()
            .filter(|schedule| {
                Range::fully_overlaps(&schedule.first, &schedule.second)
            })
            .count();

    println!("Part 1 answer: {}", fully_included_pairs_of_schedules_count);

    let overlapping_schedules_count =
        (&schedules)
            .into_iter()
            .filter(|schedule| {
                Range::overlaps(&schedule.first, &schedule.second)
            })
            .count();

    println!("Part 2 answer: {}", overlapping_schedules_count)
}
