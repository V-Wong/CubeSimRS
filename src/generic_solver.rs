use crate::prelude::{Cube, Face, Move};

pub trait Solver {
    fn solve(cube: impl Cube, moveset: &[Move], solved_state: &[Face]) -> Vec<Move>;
}