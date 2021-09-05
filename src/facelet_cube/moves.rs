use std::collections::HashMap;
use cgmath::Vector3;
use lazy_static::lazy_static;

use crate::generic_cube::{Cube, Move};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;
use crate::geometric_cube::{GeoCube, Sticker};

lazy_static! {
    pub static ref MOVE_CONVERTER: MoveConverter = {
        let mut move_converter = MoveConverter::new();
        move_converter.precompute_index_map(3);
        move_converter
    };
}

#[derive(Clone)]
pub struct FaceletMove(pub Vec<(i32, i32)>);

pub struct MoveConverter {
    moves: HashMap<(i32, Move), FaceletMove>
}

impl MoveConverter {
    pub fn new() -> Self {
        Self {
            moves: HashMap::new()
        }
    }

    pub fn convert_move(&self, size: i32, mv: Move) -> &FaceletMove {
        &self.moves[&(size, mv)]
    }

    fn precompute_index_map(&mut self, size: i32) {
        for mv in [U, R, F, D, L, B, X, Y, Z] {
            for variant in [Standard, Double, Inverse] {
                self.moves.insert((size, mv(variant)), 
                                  Self::create_move_index(size, mv(variant))
                );
            }
        }
    }

    fn create_move_index(size: i32, mv: Move) -> FaceletMove {
        let index_map = Self::create_piece_map(size);

        FaceletMove(
            GeoCube::new(size)
                   .apply_move(mv).stickers
                   .iter()
                   .map(|s| match (index_map.get(&s.destination), index_map.get(&s.position)) {
                            (Some(x), Some(y)) => (*x, *y),
                            (_, _) => panic!()
                        }                 
                    )
                    .filter(|x| x.0 != x.1)
                    .collect()
        )
    }

    fn create_piece_map(size: i32) -> HashMap<Vector3<i32>, i32> {
        let mut map = HashMap::new();
    
        let face_rotating_moves = vec![
            vec![],
            vec![X(Inverse), Y(Inverse)],
            vec![X(Inverse)],
            vec![X(Double)],
            vec![X(Inverse), Y(Standard)],
            vec![X(Inverse), Y(Double)]
        ];
    
        let mut idx = 0;
        for rotation in face_rotating_moves {
            for z in [-(size - 1), 0, size - 1] {
                for x in [-(size - 1), 0, size - 1] {
                    let first_sticker = GeoCube {
                        size: size,
                        stickers: vec![Sticker::new(size, x, size, z)],
                        mask: vec![]
                    }.apply_moves(&rotation).stickers[0];
                    map.insert(first_sticker.position, idx);
                    idx += 1;
                }
            }
        }
    
        map
    }
}