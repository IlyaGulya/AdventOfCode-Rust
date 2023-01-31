use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

#[derive(Debug)]
pub struct UnknownMove(String);

impl FromStr for Move {
    type Err = UnknownMove;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "A" => Move::ROCK,
            "B" => Move::PAPER,
            "C" => Move::SCISSORS,
            _ => return Err(UnknownMove(s.to_owned()))
        };

        Ok(result)
    }
}

impl Move {
    pub fn losing_move(opponent_move: Move) -> Move {
        match opponent_move {
            Move::ROCK => Move::SCISSORS,
            Move::PAPER => Move::ROCK,
            Move::SCISSORS => Move::PAPER,
        }
    }

    pub fn winning_move(opponent_move: Move) -> Move {
        match opponent_move {
            Move::ROCK => Move::PAPER,
            Move::PAPER => Move::SCISSORS,
            Move::SCISSORS => Move::ROCK,
        }
    }
}
