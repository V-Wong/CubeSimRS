use crate::generic_cube::{Cube, Move, Face};
use crate::generic_cube::Face::*;

use super::moves::{MOVE_CONVERTER};

#[derive(Clone)]
pub struct FaceletCube {
    size: i32,
    faces: Vec<Face>
}

impl Cube for FaceletCube {
    fn new(size: i32) -> Self {
        FaceletCube {
            size: size,
            faces: vec![
                U, U, U, U, U, U, U, U, U,
                R, R, R, R, R, R, R, R, R,
                F, F, F, F, F, F, F, F, F,
                D, D, D, D, D, D, D, D, D,
                L, L, L, L, L, L, L, L, L,
                B, B, B, B, B, B, B, B, B
            ]
        }
    } 

    fn is_solved(&self) -> bool {
        self.faces == vec![
            U, U, U, U, U, U, U, U, U,
            R, R, R, R, R, R, R, R, R,
            F, F, F, F, F, F, F, F, F,
            D, D, D, D, D, D, D, D, D,
            L, L, L, L, L, L, L, L, L,
            B, B, B, B, B, B, B, B, B
        ]
    }

    fn get_state(&self) -> Vec<Face> {
        self.faces.clone()
    }

    fn apply_move(&self, mv: Move) -> Self {
        let mut faces = self.faces.clone();

        for (x, y) in &MOVE_CONVERTER.convert_move(self.size, mv).0 {
            faces[*y as usize] = self.faces[*x as usize];
        }

        FaceletCube {
            size: self.size,
            faces: faces
        }
    }
}
