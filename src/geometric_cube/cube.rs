use crate::geometric_cube::sticker::{Sticker, Faces};
use crate::geometric_cube::turn::{Turn};
use crate::geometric_cube::moves;

pub struct Cube(Vec<Sticker>);

impl Cube {
    pub fn apply_move(&self, mv: Turn) -> Cube {
        Cube(self.0.iter().map(|sticker| sticker.rotate(mv)).collect())
    }

    pub fn apply_moves(&self, mvs: Vec<Turn>) -> Cube {
        let mut cube = Cube(self.0.clone());

        for mv in mvs {
            cube = cube.apply_move(mv);
        }

        cube
    }

    pub fn is_solved(&self) -> bool {
        self.0.iter().all(|sticker| sticker.is_solved())
    }

    pub fn get_state(&self) -> Vec<Faces> {
        let mut faces = Vec::new();

        let face_rotating_moves = vec![
            vec![],
            vec![moves::Y_MOVE, moves::X_MOVE],
            vec![moves::X_MOVE],
            vec![moves::X_MOVE, moves::X_MOVE],
            vec![moves::Y_MOVE, moves::Y_MOVE, moves::Y_MOVE, moves::X_MOVE],
            vec![moves::Y_MOVE, moves::Y_MOVE, moves::X_MOVE],
        ];

        for mvs in face_rotating_moves {
            let cube = self.apply_moves(mvs);

            let relevant_stickers = cube.0.into_iter()
                                          .filter(|sticker| matches!(sticker.get_position_face(), Faces::U))
                                          .collect();
            
            for face in Self::fill_face(relevant_stickers) {
                faces.push(face);
            }
        }
        
        faces
    }

    fn fill_face(stickers: Vec<Sticker>) -> Vec<Faces> {
        let mut faces = Vec::new();

        let mut copied_stickers = stickers.clone();
        copied_stickers.sort_by_key(|s| (-s.position.z as i64, -s.position.x as i64));

        for sticker in copied_stickers {
            faces.push(sticker.get_destination_face());
        }

        faces
    }
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for v in &self.0 {
            write!(f, "{}\n", v)?;
        }
        Ok(())
    }
}

pub fn cube3() -> Cube {
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

    Cube(stickers.to_vec())
}