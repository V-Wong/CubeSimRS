use crate::geometric_cube::sticker::Sticker;
use crate::geometric_cube::turn::{Turn, Axes};

pub struct Cube(Vec<Sticker>);

impl Cube {
    pub fn apply_move(&self, mv: Turn) -> Cube {
        Cube(self.0.iter().map(|sticker| sticker.rotate(mv)).collect())
    }

    pub fn is_solved(&self) -> bool {
        self.0.iter().all(|sticker| sticker.is_solved())
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

pub static U_MOVE: Turn = Turn { axis: Axes::Y, angle: 90.0, predicate: |sticker| sticker.position.y > 0.0 };

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