use crate::round::Round;
use crate::mv::Move;

pub struct Outcome {
    pub(crate) result: crate::result::Result,
    pub(crate) move_score: i32,
}

impl Outcome {
    pub fn of_round(round: Round) -> Outcome {
        let result = match (round.my_move, round.opponent_move) {
            (Move::PAPER, Move::ROCK) | (Move::ROCK, Move::SCISSORS) | (Move::SCISSORS, Move::PAPER) => crate::result::Result::WIN,
            (opponent, my) if opponent == my => crate::result::Result::DRAW,
            _ => crate::result::Result::LOSE,
        };

        Outcome {
            result,
            move_score: match round.my_move {
                Move::ROCK => 1,
                Move::PAPER => 2,
                Move::SCISSORS => 3,
            },
        }
    }
}
