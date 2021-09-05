use crate::generic_cube::{Cube, Move, Face};
use crate::generic_cube::Face::*;

use super::moves::{MOVE_CONVERTER};

/// A Rubik's Cube with stickers stored sequentially in a 1-dimensional array.
/// 
/// Each move is implemented as an array of index mappings of the form ``(from_idx, to_idx)``.
/// A move is then applied by swapping all pieces as specified by these index mappings.
/// 
/// Applying moves for the ``FaceletCube`` is more efficient than the ``GeoCube``, but
/// it is harder to define moves from scratch. Instead of deriving index mappings from scratch,
/// we first implement a GeoCube move, then use our conversion function to map the move
/// to a FaceletCube move.
#[derive(Clone)]
pub struct FaceletCube {
    size: i32,
    faces: Vec<Face>
}

impl Cube for FaceletCube {
    fn new(size: i32) -> Self {
        Self {
            size,
            faces: vec![
                repeat(U, size * size),
                repeat(R, size * size),
                repeat(F, size * size),
                repeat(D, size * size),
                repeat(L, size * size),
                repeat(B, size * size),
            ].concat()
        }
    }

    fn mask(size: i32, mask: &[i32]) -> Self {
        let faces = Self::new(size).faces;
        let masked_faces = faces.iter()
                                .enumerate()
                                .map(|(i, &x)| if mask.contains(&(i as i32)) { x } else { Face::X } )
                                .collect();

        Self { size, faces: masked_faces }
    }

    fn is_solved(&self) -> bool {
        let face_length = (self.size * self.size) as usize;

        let mut is_solved = true;
        for i in 0..6 {
            let face_start = i * face_length;
            let face_end = face_start + face_length;

            is_solved = is_solved && all_equal(&self.faces[face_start..face_end]);
        }

        is_solved
    }

    fn get_state(&self) -> Vec<Face> {
        self.faces.clone()
    }

    fn apply_move(&self, mv: Move) -> Self {
        let mut faces = self.faces.clone();

        for (x, y) in &MOVE_CONVERTER.convert_move(self.size, mv).0 {
            faces[*y as usize] = self.faces[*x as usize];
        }

        Self { 
            size: self.size, 
            faces
        }
    }
}

fn repeat<T: Clone>(element: T, count: i32) -> Vec<T> {
    std::iter::repeat(element).take(count as usize).collect()
}

fn all_equal<T: Clone + PartialEq>(arr: &[T]) -> bool {
    arr.iter().all(|x| *x == arr[0])
}