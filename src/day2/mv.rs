#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Move {
    pub fn by_str(string: &str) -> Move {
        match string {
            "A" => Move::ROCK,
            "B" => Move::PAPER,
            "C" => Move::SCISSORS,
            _ => panic!("Unknown move {}", string)
        }
    }

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
