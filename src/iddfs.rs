use std::collections::HashMap;
use std::mem::discriminant;

use crate::generic_cube::{Cube, Face, Move};
use crate::facelet_cube::{FaceletCube};

pub struct Solver {
    candidate_moves: Vec<Move>,
    pruning_table: HashMap<Vec<Face>, i32>,
    pruning_depth: i32
}

impl Solver {
    pub fn new(candidate_moves: Vec<Move>, pruning_table: HashMap<Vec<Face>, i32>, pruning_depth: i32) -> Self {
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

pub fn solve(cube: &impl Cube, 
             moveset: &[Move], 
             solved_state: &[Face],
             limit: i32) -> Option<Vec<Move>> {
    iddfs(
        cube, 
        &Solver::new(moveset.to_vec(), gen_pruning_table(vec![FaceletCube::new(3)], limit, moveset), limit), 
        limit
    )
}

pub fn iddfs(cube: &impl Cube, 
             solver: &Solver,
             limit: i32) -> Option<Vec<Move>> {
    for i in 0..=limit {
        if let Some(sol) = dfs(cube, solver, &mut vec![], i) {
            return Some(sol);
        }
    }

    None
}

pub fn dfs(cube: &impl Cube, 
           solver: &Solver,
           solution: &mut Vec<Move>,
           depth_remaining: i32) -> Option<Vec<Move>> {
    if solver.is_solved(cube) {
        return Some(solution.to_vec());
    }

    let lower_bound = solver.lower_bound(cube);

    if lower_bound > depth_remaining {
        return None
    }

    for mv in &solver.candidate_moves {
        if let Some(last_mv) = solution.last() {
            if discriminant(last_mv) == discriminant(&mv) {
                continue;
            }
        }

        solution.push(*mv);

        let result = dfs(
            &cube.apply_move(*mv),
            solver,
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

pub fn gen_pruning_table(starting_cubes: Vec<impl Cube>, depth: i32, moveset: &[Move]) -> HashMap<Vec<Face>, i32> {
    let mut pruning_table: HashMap<Vec<Face>, i32> = HashMap::new();
    let mut previous_frontier = starting_cubes.clone();

    for cube in starting_cubes {
        pruning_table.insert(cube.get_state(), 0);
    }

    for i in 1..=depth {
        let mut frontier = vec![];

        for cube in &previous_frontier {
            for mv in moveset {
                let new_cube = cube.apply_move(*mv);
                if !pruning_table.contains_key(&new_cube.get_state()) {
                    pruning_table.insert(new_cube.get_state(), i);
                    frontier.push(new_cube);
                }
            }
        }

        previous_frontier = frontier;
    } 

    pruning_table
}