use rustc_hash::FxHashMap;
use std::mem::discriminant;

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

fn dfs(cube: &impl Cube, 
       solver: &Solver, 
       solution: &mut Vec<Move>,
       depth_remaining: i32) -> Option<Vec<Move>> {
    if solver.is_solved(cube) {
        return Some(solution.to_vec());
    }

    if depth_remaining == 0 {
        return None;
    }

    for mv in &solver.candidate_moves {
        if let Some(last_mv) = solution.last() {
            if discriminant(last_mv) == discriminant(mv) {
                continue;
            }
        }

        solution.push(*mv);

        let result = dfs(
            &cube.apply_move(*mv),
            &solver,
            solution,
            depth_remaining - 1
        );

        solution.pop();

        if result.is_some() {
            return result;
        }
    }

    None
}