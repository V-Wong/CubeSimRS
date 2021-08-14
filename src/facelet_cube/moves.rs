use std::collections::HashMap;
use cgmath::Vector3;
use lazy_static::lazy_static;

use crate::generic_cube::{Cube, Move};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;
use crate::geometric_cube::{GeoCube, Sticker, cube3};

#[derive(Clone)]
pub struct FaceletMove(pub Vec<(i32, i32)>);

pub fn convert_move(mv: Move) -> &'static FaceletMove {
    match mv {
        U(variant) => &U_MOVE,
        R(variant) => &R_MOVE,
        F(variant) => &F_MOVE,
        L(variant) => &L_MOVE,
        D(variant) => &D_MOVE,
        B(variant) => &B_MOVE,
        _ => &F_MOVE
    }
}

fn from_geometric_move(mv: Move) -> Vec<(i32, i32)> {
    let index_map: HashMap<Vector3<i32>, i32> = create_index_conversion_map();

    cube3().apply_move(mv).0
           .iter()
           .map(|s|
                match (index_map.get(&s.destination), index_map.get(&s.position)) {
                    (Some(x), Some(y)) => return (x.clone(), y.clone()),
                    (_, _) => panic!()
                }                 
           )
           .filter(|x| x.0 != x.1)
           .collect()
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
    pub static ref U_MOVE: FaceletMove = FaceletMove(from_geometric_move(U(Standard)));
    pub static ref R_MOVE: FaceletMove = FaceletMove(from_geometric_move(R(Standard)));
    pub static ref F_MOVE: FaceletMove = FaceletMove(from_geometric_move(F(Standard)));
    pub static ref D_MOVE: FaceletMove = FaceletMove(from_geometric_move(D(Standard)));
    pub static ref L_MOVE: FaceletMove = FaceletMove(from_geometric_move(L(Standard)));
    pub static ref B_MOVE: FaceletMove = FaceletMove(from_geometric_move(B(Standard)));
}