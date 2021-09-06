use std::collections::HashMap;
use cached::proc_macro::cached;
use cgmath::Vector3;

use crate::generic_cube::{Cube, Move};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;
use crate::geometric_cube::{GeoCube, Sticker};

#[derive(Clone)]
pub struct FaceletMove(pub Vec<(i32, i32)>);

#[cached]
pub fn convert_move(size: i32, mv: Move) -> FaceletMove {
    let index_map = create_piece_map(size);

    FaceletMove(
        GeoCube::new(size)
                .apply_move(mv)
                .stickers
                .iter()
                .map(|s| (index_map[(&s.destination)], index_map[(&s.position)]))
                .filter(|x| x.0 != x.1)
                .collect()
    )
}

#[cached]
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
                    size,
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