use crate::generic_cube::{Cube, Move, Face};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

use super::sticker::{Sticker};
use super::moves::{GeometricMove};

/// A Rubik's Cube with each of its facelets represented as a Sticker.
/// 
/// Each move is implemented as a rotation matrix and hence a move is applied
/// by multiplying each of the relevant vectors with the matrix.
/// 
/// This implementation of a Rubik's Cube is very programmatic but suffers
/// from poor performance due to the expensive nature of matrix multiplication.
/// This implementation should not be used directly, instead it should only be
/// used to bootstrap more efficient implementations like the FaceletCube.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct GeoCube {
    pub size: i32,
    pub stickers: Vec<Sticker>,
}

impl Cube for GeoCube {
    fn new(size: i32) -> Self {
        Self { 
            size, 
            stickers: Self::generate_stickers(size)
        }
    }

    fn size(&self) -> i32 {
        self.size
    }

    fn apply_move(&self, mv: Move) -> Self {
        Self {
            stickers: self.stickers.iter()
                          .map(|s| s.rotate(GeometricMove::from(mv)))
                          .collect(),
            ..self.clone()
        }
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
            let cube = self.apply_moves(&mvs);

            let relevant_stickers = cube.stickers.into_iter()
                                        .filter(|s| matches!(s.get_position_face(), Face::U))
                                        .collect::<Vec<Sticker>>();
            
            for sticker in Self::sort_stickers(&relevant_stickers) {
                faces.push(sticker.get_destination_face());
            }
        }
        
        faces
    }

    fn mask(&self, mask: &dyn Fn(i32, Face) -> Face) -> Self {
        let masked_stickers = self.stickers
                                  .iter()
                                  .map(|s| Sticker { destination_face: mask(s.initial_index, s.get_destination_face()), ..*s })
                                  .collect::<Vec<_>>();

        Self { stickers: masked_stickers, ..*self }
    }
}

impl GeoCube {
    fn generate_stickers(size: i32) -> Vec<Sticker> {
        let mut stickers = Vec::new();

        for face in [-size, size] {
            for p1 in Self::range(size) {
                for p2 in Self::range(size) {
                    stickers.push(Sticker::new(size, face, p1, p2));
                    stickers.push(Sticker::new(size, p1, face, p2));
                    stickers.push(Sticker::new(size, p1, p2, face));
                }
            }
        }

        Self::set_sticker_initial_index(size, stickers)
    }

    fn set_sticker_initial_index(size: i32, stickers: Vec<Sticker>) -> Vec<Sticker> {
        let mut result = Vec::new();

        let face_rotating_moves = vec![
            vec![],
            vec![Y(Standard), X(Standard)],
            vec![X(Standard)],
            vec![X(Double)],
            vec![Y(Inverse), X(Standard)],
            vec![Y(Double), X(Standard)],
        ];

        let mut idx = 0;
        for mvs in face_rotating_moves {
            let cube = Self { 
                size, 
                stickers: stickers.clone()
            }.apply_moves(&mvs);

            let mut relevant_stickers = cube.stickers.into_iter()
                                            .filter(|s| matches!(s.get_position_face(), Face::U))
                                            .collect::<Vec<Sticker>>();

            relevant_stickers.sort_by_key(|s| (s.position.z as i64, s.position.x as i64));
            
            for sticker in relevant_stickers.iter() {
                result.push(Sticker {
                    size,
                    position: sticker.destination,
                    destination: sticker.destination,
                    destination_face: Sticker::get_face(size, sticker.destination.x, sticker.destination.y, sticker.destination.z),
                    initial_index: idx,
                });

                idx += 1;
            }
        }

        result
    }

    fn sort_stickers(stickers: &[Sticker]) -> Vec<Sticker> {
        let mut cloned_stickers = stickers.to_owned();
        cloned_stickers.sort_by_key(|s| (s.position.z as i64, s.position.x as i64));
        cloned_stickers
    }

    /// Returns the range of facelet center coordinates along an arbitrary axis.
    pub fn range(size: i32) -> Vec<i32> {
        (-size + 1 ..= size - 1).step_by(2).collect()
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