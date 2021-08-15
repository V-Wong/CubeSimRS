use std::collections::HashMap;
use cgmath::Vector3;
use lazy_static::lazy_static;

use crate::generic_cube::{Cube, Move};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;
use crate::geometric_cube::{GeoCube, Sticker};

#[derive(Clone)]
pub struct FaceletMove(pub Vec<(i32, i32)>);

impl From<Move> for FaceletMove {
    fn from(mv: Move) -> Self {
        let index_map: HashMap<Vector3<i32>, i32> = create_index_conversion_map();

        FaceletMove(
            GeoCube::new().apply_move(mv).0
                          .iter()
                          .map(|s|
                                  match (index_map.get(&s.destination), index_map.get(&s.position)) {
                                      (Some(x), Some(y)) => (*x, *y),
                                      (_, _) => panic!()
                                  }                 
                          )
                          .filter(|x| x.0 != x.1)
                          .collect()
        )
    }
}

pub fn convert_move(mv: Move) -> &'static FaceletMove {
    &FACELET_MOVES[&mv]
}

fn create_index_conversion_map() -> HashMap<Vector3<i32>, i32>{
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
        for z in [-2, 0, 2] {
            for x in [-2, 0, 2] {
                let first_sticker = GeoCube(vec![Sticker::new(x, 3, z)]).apply_moves(rotation.clone()).0[0];
                map.insert(first_sticker.position, idx);
                idx += 1;
            }
        }
    }

    map
}

lazy_static! {
    static ref FACELET_MOVES: HashMap<Move, FaceletMove> = {
        let mut map = HashMap::new();

        for mv in [U, R, F, D, L, B] {
            for move_variant in [Standard, Double, Inverse] {
                map.insert(mv(move_variant), FaceletMove::from(mv(move_variant)));
            }
        } 

        map
    };
}