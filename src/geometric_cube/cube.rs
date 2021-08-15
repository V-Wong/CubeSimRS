use crate::generic_cube::{Cube, Move, Face};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

use super::sticker::{Sticker};
use super::moves::{GeometricMove};

#[derive(Clone)]
pub struct GeoCube(pub Vec<Sticker>);

impl Cube for GeoCube {
    fn new() -> Self {
        let mut stickers = Vec::new();

        for face in [-3, 3] {
            for p1 in [-2, 0, 2] {
                for p2 in [-2, 0, 2] {
                    stickers.push(Sticker::new(face, p1, p2));
                    stickers.push(Sticker::new(p1, face, p2));
                    stickers.push(Sticker::new(p1, p2, face));
                }
            }
        }
    
        GeoCube(stickers.to_vec())
    }

    fn apply_move(&self, mv: Move) -> Self {
        GeoCube(self.0.iter()
                      .map(|sticker| sticker.rotate(GeometricMove::from(mv)))
                      .collect()
        )
    }

    fn is_solved(&self) -> bool {
        self.0.iter().all(|sticker| sticker.is_solved())
    }

    fn get_state(&self) -> Vec<Face> {
        let mut faces = Vec::new();

        let face_rotating_moves = vec![
            vec![],
            vec![Y(Standard), X(Standard)],
            vec![X(Standard)],
            vec![X(Double)],
            vec![Y(Inverse), X(Standard)],
            vec![Y(Double), X(Standard)],
        ];

        for mvs in face_rotating_moves {
            let cube = self.apply_moves(mvs);

            let mut relevant_stickers = cube.0.into_iter()
                                            .filter(|s| matches!(s.get_position_face(), Face::U))
                                            .collect();
            
            for face in Self::fill_face(&mut relevant_stickers) {
                faces.push(face);
            }
        }
        
        faces
    }
}

impl GeoCube {
    fn fill_face(stickers: &mut Vec<Sticker>) -> Vec<Face> {
        stickers.sort_by_key(|s| (s.position.z as i64, s.position.x as i64));
        stickers.iter().map(Sticker::get_destination_face).collect()
    }
}

impl std::fmt::Display for GeoCube {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for v in &self.0 {
            writeln!(f, "{}", v)?;
        }
        Ok(())
    }
}