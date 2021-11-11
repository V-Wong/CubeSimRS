use lazy_static::lazy_static;

use crate::generic_cube::{Cube, Move, Face, CubeSize};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

use super::sticker::{Sticker};
use super::moves::{GeometricMove};

lazy_static! {
    static ref FACE_ROTATING_MOVES: Vec<Vec<Move>> = vec![
        vec![],
        vec![Y(Standard), X(Standard)],
        vec![X(Standard)],
        vec![X(Double)],
        vec![Y(Inverse), X(Standard)],
        vec![Y(Double), X(Standard)],
    ];
}

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
    pub(crate) size: CubeSize,
    pub(crate) stickers: Vec<Sticker>,
}

impl Cube for GeoCube {
    fn new(size: CubeSize) -> Self {
        Self { 
            size, 
            stickers: Self::generate_stickers(size)
        }
    }

    fn size(&self) -> CubeSize {
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

    fn state(&self) -> Vec<Face> {
        let mut faces = Vec::new();

        for mvs in &*FACE_ROTATING_MOVES {
            let rotated_cube = self.apply_moves(&mvs);
            let top_layer_stickers = rotated_cube.top_layer_stickers();
            
            for sticker in top_layer_stickers {
                faces.push(sticker.destination_face());
            }
        }
        
        faces
    }

    fn mask(&self, mask: &dyn Fn(CubeSize, Face) -> Face) -> Self {
        let masked_stickers = self.stickers
                                  .iter()
                                  .map(|s| Sticker { destination_face: mask(s.initial_index, s.destination_face()), ..*s })
                                  .collect::<Vec<_>>();

        Self { stickers: masked_stickers, ..*self }
    }
}

impl GeoCube {
    fn generate_stickers(size: CubeSize) -> Vec<Sticker> {
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

    fn set_sticker_initial_index(size: CubeSize, stickers: Vec<Sticker>) -> Vec<Sticker> {
        let mut result = Vec::new();

        for (idx, mvs) in (&*FACE_ROTATING_MOVES).iter().enumerate() {
            let rotated_cube = Self { size, stickers: stickers.clone() }.apply_moves(&mvs);
            let top_layer_stickers = rotated_cube.top_layer_stickers();

            for sticker in top_layer_stickers.iter() {
                result.push(Sticker {
                    size,
                    position: sticker.destination,
                    destination: sticker.destination,
                    destination_face: Sticker::face(size, sticker.destination.x, sticker.destination.y, sticker.destination.z),
                    initial_index: idx as i32,
                });
            }
        }

        result
    }

    fn top_layer_stickers(&self) -> Vec<Sticker> {
        let mut top_layer_stickers = self.stickers
            .to_owned()
            .into_iter()
            .filter(|s| matches!(s.position_face(), Face::U))
            .collect::<Vec<_>>();

        top_layer_stickers.sort_by_key(|s| (s.position.z as i64, s.position.x as i64));
        top_layer_stickers
    }

    /// Returns the range of facelet center coordinates along an arbitrary axis.
    pub fn range(size: CubeSize) -> Vec<CubeSize> {
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