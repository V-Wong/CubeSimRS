use crate::generic_cube::{Cube, Face, Move, MoveVariant, all_moves};
use crate::generic_cube::{sticker_index as S};
use crate::facelet_cube::FaceletCube;
use crate::generic_solver::{Solver, ida_star, gen_pruning_table};

pub fn phase1(cube: &impl Cube) -> Option<Vec<Move>> {
    use Face::*;

    let g1_mask = vec![
        S(3, U, 2), S(3, U, 4), S(3, U, 6), S(3, U, 8),
        S(3, D, 2), S(3, D, 4), S(3, D, 6), S(3, D, 8),
        S(3, F, 4), S(3, F, 6), S(3, B, 4), S(3, B, 6)
    ];

    let mask = |i: i32, _| if g1_mask.contains(&i) { U } else { X };

    let moves = all_moves(3);
    let search_limit = 10;
    let pruning_depth = 7;
    let pruning_table = gen_pruning_table(vec![FaceletCube::new(3).mask(&mask)], pruning_depth, &moves);

    let solver = Solver::new(moves, pruning_table, pruning_depth);

    ida_star(&cube.mask(&mask), &solver, search_limit)
}

pub fn phase2(cube: &impl Cube) -> Option<Vec<Move>> {
    use Face::*;
    use MoveVariant::*;

    let co_pieces = vec![
        S(3, U, 1), S(3, U, 3), S(3, U, 7), S(3, U, 9),
        S(3, D, 1), S(3, D, 3), S(3, D, 7), S(3, D, 9)
    ];

    let eo_ud_pieces = vec![
        S(3, U, 2), S(3, U, 4), S(3, U, 6), S(3, U, 8),
        S(3, D, 2), S(3, D, 4), S(3, D, 6), S(3, D, 8)
    ];

    let eo_e_piecs = vec![
        S(3, F, 4), S(3, F, 6), S(3, B, 4), S(3, B, 6)
    ];

    let mask = |i: i32, _| 
        if eo_ud_pieces.contains(&i) || co_pieces.contains(&i) { X } 
        else { if eo_e_piecs.contains(&i) { U } else { R } };

    let moves = vec![
        Move::U(Standard), Move::U(Inverse), Move::U(Double), 
        Move::D(Standard), Move::D(Inverse), Move::D(Double),  
        Move::L(Standard), Move::L(Inverse), Move::L(Double), 
        Move::R(Standard), Move::R(Inverse), Move::R(Double),  
        Move::F(Double), Move::B(Double) 
    ];

    let search_limit = 10;
    let pruning_depth = 5;
    let pruning_table = gen_pruning_table(vec![FaceletCube::new(3).mask(&mask)], pruning_depth, &moves);

    let solver = Solver::new(moves, pruning_table, pruning_depth);

    ida_star(&cube.mask(&mask), &solver, search_limit)
}