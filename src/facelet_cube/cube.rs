use crate::generic_cube::{Cube, Move, Face};
use crate::generic_cube::Face::*;

use super::moves;

#[derive(Clone)]
pub struct FaceletCube(Vec<Face>);

impl Cube for FaceletCube {
    fn is_solved(&self) -> bool {
        self.0 == vec![
            U, U, U, U, U, U, U, U, U,
            R, R, R, R, R, R, R, R, R,
            F, F, F, F, F, F, F, F, F,
            D, D, D, D, D, D, D, D, D,
            L, L, L, L, L, L, L, L, L,
            B, B, B, B, B, B, B, B, B
        ]
    }

    fn get_state(&self) -> Vec<Face> {
        return self.0.clone();
    }

    fn apply_move(&self, mv: Move) -> Self {
        let mut faces = self.0.clone();

        for (x, y) in moves::from_geometric_move(mv) {
            faces[y as usize] = self.0[x as usize];
        }

        FaceletCube(faces)
    }
}

pub fn cube3() -> FaceletCube {
    FaceletCube(vec![
        U, U, U, U, U, U, U, U, U,
        R, R, R, R, R, R, R, R, R,
        F, F, F, F, F, F, F, F, F,
        D, D, D, D, D, D, D, D, D,
        L, L, L, L, L, L, L, L, L,
        B, B, B, B, B, B, B, B, B
    ])
}