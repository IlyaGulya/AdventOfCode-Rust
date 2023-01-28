use crate::mv::Move;

pub struct Strategy {
    pub(crate) opponent_move: Move,
    pub(crate) my_outcome: crate::result::Result,
}
