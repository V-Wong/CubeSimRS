use crate::prelude::{Cube, Face, Move};

/// A Rubik's Cube solver.
pub trait Solver {
    /// Solves the Cube into the given solve state using the given moveset.
    /// 
    /// # Arguments
    ///
    /// * `cube` - The Cube to be solved.
    /// * `moveset` - The moves to be used in the solution.
    /// * `solved_state` - The desired end state of a solution.
    fn solve(cube: impl Cube, moveset: &[Move], solved_state: &[Face]) -> Vec<Move>;
}