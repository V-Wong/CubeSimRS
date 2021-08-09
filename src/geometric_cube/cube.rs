use crate::generic_cube::{Cube, Move, Face};

use super::sticker::{Sticker};
use super::moves;

#[derive(Clone)]
pub struct GeoCube(Vec<Sticker>);

impl Cube for GeoCube {
    fn apply_move(&self, mv: Move) -> Self {
        let converted_move = match mv {
            Move::U => moves::U_MOVE,
            Move::R => moves::R_MOVE,
            Move::F => moves::F_MOVE,
            Move::L => moves::L_MOVE,
            Move::D => moves::D_MOVE,
            Move::B => moves::B_MOVE,
            Move::X => moves::X_MOVE,
            Move::Y => moves::Y_MOVE,
            Move::Z => moves::Z_MOVE
        };

        GeoCube(self.0.iter().map(|sticker| sticker.rotate(converted_move)).collect())
    }

    fn is_solved(&self) -> bool {
        self.0.iter().all(|sticker| sticker.is_solved())
    }

    fn get_state(&self) -> Vec<Face> {
        let mut faces = Vec::new();

        let face_rotating_moves = vec![
            vec![],
            vec![Move::Y, Move::X],
            vec![Move::X],
            vec![Move::X, Move::X],
            vec![Move::Y, Move::Y, Move::Y, Move::X],
            vec![Move::Y, Move::Y, Move::X],
        ];

        for mvs in face_rotating_moves {
            let cube = self.apply_moves(mvs);

            let relevant_stickers = cube.0.into_iter()
                                          .filter(|sticker| matches!(sticker.get_position_face(), Face::U))
                                          .collect();
            
            for face in Self::fill_face(relevant_stickers) {
                faces.push(face);
            }
        }
        
        faces
    }
}

impl GeoCube {
    fn fill_face(stickers: Vec<Sticker>) -> Vec<Face> {
        let mut faces = Vec::new();

        let mut copied_stickers = stickers.clone();
        copied_stickers.sort_by_key(|s| (-s.position.z as i64, -s.position.x as i64));

        for sticker in copied_stickers {
            faces.push(sticker.get_destination_face());
        }

        faces
    }
}

impl std::fmt::Display for GeoCube {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for v in &self.0 {
            write!(f, "{}\n", v)?;
        }
        Ok(())
    }
}

pub fn cube3() -> GeoCube {
    let mut stickers = Vec::new();

    for face in [-3.0, 3.0] {
        for p1 in [-2.0, 0.0, 2.0] {
            for p2 in [-2.0, 0.0, 2.0] {
                stickers.push(Sticker::new(face, p1, p2));
                stickers.push(Sticker::new(p1, face, p2));
                stickers.push(Sticker::new(p1, p2, face));
            }
        }
    }

    GeoCube(stickers.to_vec())
}