use rustc_hash::FxHashMap;
use crate::prelude::{Cube, Face, Move};

pub struct Solver {
    pub candidate_moves: Vec<Move>,
    pub pruning_table: FxHashMap<Vec<Face>, i32>,
    pub pruning_depth: i32
}

impl Solver {
    pub fn new(candidate_moves: Vec<Move>, pruning_table: FxHashMap<Vec<Face>, i32>, pruning_depth: i32) -> Self {
        Self {
            candidate_moves,
            pruning_table,
            pruning_depth
        }
    }

    pub fn is_solved(&self, cube: &impl Cube) -> bool {
        match self.pruning_table.get(&cube.get_state()) {
            Some(0) => return true,
            _ => false
        }
    }

    pub fn lower_bound(&self, cube: &impl Cube) -> i32 {
        match self.pruning_table.get(&cube.get_state()) {
            Some(n) => *n,
            _ => self.pruning_depth + 1
        }
    }
}