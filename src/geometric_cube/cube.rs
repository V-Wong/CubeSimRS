use crate::generic_cube::{Cube, Move, Face};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

use super::sticker::{Sticker};
use super::moves::{GeometricMove};

#[derive(Clone)]
pub struct GeoCube {
    pub size: i32,
    pub stickers: Vec<Sticker>,
    mask: Vec<i32>
}

impl Cube for GeoCube {
    fn new(size: i32) -> Self {
        let mut stickers = Vec::new();

        for face in [-size, size] {
            for p1 in [-(size - 1), 0, size - 1] {
                for p2 in [-(size - 1), 0, size - 1] {
                    stickers.push(Sticker::new(size, face, p1, p2));
                    stickers.push(Sticker::new(size, p1, face, p2));
                    stickers.push(Sticker::new(size, p1, p2, face));
                }
            }
        }
    
        GeoCube { size: size, stickers: stickers.to_vec(), mask: vec![] }
    }

    fn mask(size: i32, mask: &Vec<i32>) -> Self {
        GeoCube { mask: mask.clone(), ..Self::new(size) }
    }

    fn apply_move(&self, mv: Move) -> Self {
        Self {
            size: self.size,
            stickers: self.stickers.iter()
                          .map(|s| s.rotate(GeometricMove::from(mv)))
                          .collect(),
            mask: Vec::new()
        }
    }

    fn is_solved(&self) -> bool {
        self.stickers.iter().all(|sticker| sticker.is_solved())
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

            let mut relevant_stickers = cube.stickers.into_iter()
                                            .filter(|s| matches!(s.get_position_face(), Face::U))
                                            .collect();
            
            for face in Self::fill_face(&mut relevant_stickers) {
                faces.push(face);
            }
        }
        
        faces.iter()
             .enumerate()
             .map(|(i, &x)| if self.mask.contains(&(i as i32)) { x } else { Face::X } )
             .collect()
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
        for v in &self.stickers {
            writeln!(f, "{}", v)?;
        }
        Ok(())
    }
}