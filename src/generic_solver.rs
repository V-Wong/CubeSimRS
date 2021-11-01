use rustc_hash::FxHashMap;
use std::mem::discriminant;

use crate::generic_cube::{Cube, Face, Move};
use crate::facelet_cube::FaceletCube;

pub struct Solver {
    pub candidate_moves: Vec<Move>,
    pub pruning_table: PruningTable
}

pub struct PruningTable {
    pruning_table: FxHashMap<Vec<Face>, i32>,
    depth: i32 
}

impl PruningTable {
    pub fn new(starting_cubes: Vec<impl Cube>, depth: i32, moveset: &[Move]) -> Self {
        let mut pruning_table: FxHashMap<Vec<Face>, i32> = FxHashMap::default();
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
    
        Self {
            pruning_table,
            depth
        }
    }

    pub fn from_existing_table(other: &PruningTable, depth: i32, moveset: &[Move]) -> Self {
        Self::new(
            other.pruning_table.keys().map(|faces| FaceletCube::from(faces.clone())).collect::<Vec<_>>(),
            depth,
            moveset
        )
    }

    fn get(&self, k: &Vec<Face>) -> Option<&i32> {
        self.pruning_table.get(k)
    }
}

impl Solver {
    pub fn new(candidate_moves: Vec<Move>, pruning_table: PruningTable) -> Self {
        Self {
            candidate_moves,
            pruning_table
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
            _ => self.pruning_table.depth + 1
        }
    }
}

pub fn ida_star(cube: &impl Cube, 
         solver: &Solver,
         limit: i32) -> Option<Vec<Move>> {
    for i in 0..=limit {
        if let Some(sol) = dfs(cube, solver, &mut vec![], i) {
            return Some(sol);
        }
    }

    None
}

fn dfs(cube: &impl Cube, 
       solver: &Solver, 
       solution: &mut Vec<Move>,
       depth_remaining: i32) -> Option<Vec<Move>> {
    if solver.is_solved(cube) {
        return Some(solution.to_vec());
    }

    if solver.lower_bound(cube) > depth_remaining {
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