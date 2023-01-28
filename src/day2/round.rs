use crate::mv::Move;

#[derive(Debug)]
pub struct Round {
    pub(crate) opponent_move: Move,
    pub(crate) my_move: Move,
}
