use itertools::Itertools;
use crate::mv::Move;
use crate::stack::Stack;

pub struct CrateMover {
    pub version: CrateMoverVersion,
}

pub enum CrateMoverVersion {
    Version9000,
    Version9001,
}

impl CrateMover {
    pub fn execute_program(self: &CrateMover, stacks: &mut Vec<Stack>, program: &Vec<Move>) {
        for mv in program {
            let source_stack =
                &mut stacks
                    .get_mut(mv.from_stack as usize)
                    .unwrap()
                    .items;

            let start = source_stack.len() - (mv.amount as usize);
            let end = source_stack.len();

            let mut drain =
                source_stack
                    .drain(start..end)
                    .collect_vec();

            match self.version {
                CrateMoverVersion::Version9000 => {
                    drain.reverse()
                }
                CrateMoverVersion::Version9001 => {
                    // no-op. CrateMover9001 can move several
                    // blocks at once and does not change the order
                }
            }

            stacks
                .get_mut(mv.to_stack as usize)
                .unwrap()
                .items
                .append(&mut drain);
        }
    }
}
