use lazy_static::lazy_static;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Move {
    pub(crate) amount: u32,
    pub(crate) from_stack: u32,
    pub(crate) to_stack: u32,
}

pub fn parse_moves(input: &String) -> Vec<Move> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (?P<amount>\d+) from (?P<from_stack>\d+) to (?P<to_stack>\d+)").unwrap();
    }

    (&input)
        .lines()
        .filter(|line| line.starts_with("move"))
        .map(|line| {
            let cap = RE.captures(line).unwrap();

            let amount = cap.name("amount")?;
            let from_stack = cap.name("from_stack")?;
            let to_stack = cap.name("to_stack")?;

            let from_stack: u32 = from_stack.as_str().parse().unwrap();
            let to_stack: u32 = to_stack.as_str().parse().unwrap();

            Some(
                Move {
                    amount: amount.as_str().parse().unwrap(),
                    from_stack: from_stack - 1,
                    to_stack: to_stack - 1,
                }
            )
        })
        .map(|mv| mv.unwrap())
        .collect_vec()
}
