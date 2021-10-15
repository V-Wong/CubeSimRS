use std::mem::discriminant;

use crate::generic_cube::{Cube, Face, Move};

pub fn iddfs(cube: &impl Cube, 
             moveset: &[Move], 
             solved_state: &[Face],
             limit: i32) -> Option<Vec<Move>> {
    for i in 0..=limit {
        if let Some(sol) = dfs(cube, moveset, solved_state, &mut vec![], i) {
            return Some(sol);
        }
    }

    None
}

pub fn dfs(cube: &impl Cube, 
           moveset: &[Move], 
           solved_state: &[Face], 
           solution: &mut Vec<Move>,
           depth_remaining: i32) -> Option<Vec<Move>> {
    if cube.get_state() == solved_state {
        return Some(solution.to_vec());
    }

    if depth_remaining == 0 {
        return None;
    }

    for mv in moveset {
        if let Some(last_mv) = solution.last() {
            if discriminant(last_mv) == discriminant(mv) {
                continue;
            }
        }

        solution.push(*mv);

        let result = dfs(
            &cube.apply_move(*mv),
            moveset,
            solved_state,
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

