use crate::generic_cube::{Cube, Face, Move, MoveVariant, all_moves};
use crate::generic_cube::{sticker_index as S};
use crate::facelet_cube::FaceletCube;
use crate::generic_solver::{Solver, PruningTable, ida_star};

pub fn solve(cube: &impl Cube) -> Option<Vec<Move>> {
    let mut solution = vec![];
    let mut cube = cube.clone();

    for phase in [phase1, phase2, phase3, phase4] {
        let mut phase_solution = phase(&cube)?;
        cube = cube.apply_moves(&phase_solution);
        solution.append(&mut phase_solution);
    }

    Some(solution)
}

pub fn phase1(cube: &impl Cube) -> Option<Vec<Move>> {
    use Face::*;

    let g1_mask = [
        S(3, U, 2), S(3, U, 4), S(3, U, 6), S(3, U, 8),
        S(3, D, 2), S(3, D, 4), S(3, D, 6), S(3, D, 8),
        S(3, F, 4), S(3, F, 6), S(3, B, 4), S(3, B, 6)
    ];

    let mask = |i: i32, _| if g1_mask.contains(&i) { U } else { X };

    let moves = all_moves(3);
    let search_limit = 10;
    let pruning_depth = 7;
    let pruning_table = PruningTable::new(&[FaceletCube::new(3).mask(&mask)], pruning_depth, &moves);

    let solver = Solver::new(moves, pruning_table);

    ida_star(&cube.mask(&mask), &solver, search_limit)
}

pub fn phase2(cube: &impl Cube) -> Option<Vec<Move>> {
    use Face::*;
    use MoveVariant::*;

    let co_pieces = [
        S(3, U, 1), S(3, U, 3), S(3, U, 7), S(3, U, 9),
        S(3, D, 1), S(3, D, 3), S(3, D, 7), S(3, D, 9)
    ];

    let eo_ud_pieces = [
        S(3, U, 2), S(3, U, 4), S(3, U, 6), S(3, U, 8),
        S(3, D, 2), S(3, D, 4), S(3, D, 6), S(3, D, 8)
    ];

    let eo_e_piecs = [
        S(3, F, 4), S(3, F, 6), S(3, B, 4), S(3, B, 6)
    ];

    let mask = |i: i32, _| 
        if eo_ud_pieces.contains(&i) || co_pieces.contains(&i) { X } 
        else if eo_e_piecs.contains(&i) { U } 
        else { R };

    let moves = vec![
        Move::U(Standard), Move::U(Inverse), Move::U(Double), 
        Move::D(Standard), Move::D(Inverse), Move::D(Double),  
        Move::L(Standard), Move::L(Inverse), Move::L(Double), 
        Move::R(Standard), Move::R(Inverse), Move::R(Double),  
        Move::F(Double), Move::B(Double) 
    ];

    let search_limit = 10;
    let pruning_depth = 5;
    let pruning_table = PruningTable::new(&[FaceletCube::new(3).mask(&mask)], pruning_depth, &moves);

    let solver = Solver::new(moves, pruning_table);

    ida_star(&cube.mask(&mask), &solver, search_limit)
}

pub fn phase3(cube: &impl Cube) -> Option<Vec<Move>>  {
    use Face::*;
    use MoveVariant::*;

    let cp_pieces = [U, D, F, B, L, R].iter()
                                      .map(|f| [1, 3, 7, 9].map(|x| S(3, *f, x)))
                                      .collect::<Vec<_>>()
                                      .concat();

    let ep_pieces = [F, B, L, R].iter()
                                .map(|f| [2, 4, 6, 8].map(|x| S(3, *f, x)))
                                .collect::<Vec<_>>()
                                .concat();

    let face = |f| if f == B { F }
                   else if f == L { R }
                   else { f };

    let mask = |i: i32, _| if cp_pieces.contains(&i) { [U, R, F, D, L, B][(i / 9) as usize] }
                           else if ep_pieces.contains(&i) { face([U, R, F, D, L, B][(i / 9) as usize]) }
                           else { X };

    let moves = vec![
        Move::U(Standard), Move::U(Inverse), Move::U(Double),
        Move::D(Standard), Move::D(Inverse), Move::D(Double),
        Move::F(Double), Move::B(Double), Move::L(Double), Move::R(Double)
    ];

    let solved_states_viewed_in_g2 = PruningTable::new(
        &[FaceletCube::new(3).mask(&mask)], 
        10,
        &vec![Move::U(Double), Move::D(Double), Move::F(Double), Move::B(Double), Move::L(Double), Move::R(Double)]
    );

    let search_limit = 13;
    let pruning_depth = 5;
    let pruning_table = PruningTable::from_existing_table(&solved_states_viewed_in_g2, pruning_depth, &moves);

    let solver = Solver::new(moves, pruning_table);

    ida_star(&cube.mask(&mask), &solver, search_limit)
}

pub fn phase4(cube: &impl Cube) -> Option<Vec<Move>> {
    use MoveVariant::*;

    let moves = vec![
        Move::U(Double), Move::D(Double), Move::F(Double),
        Move::B(Double), Move::L(Double), Move::R(Double)
    ];
    let search_limit = 14;
    let pruning_depth = 6;
    let pruning_table = PruningTable::new(&[FaceletCube::new(3)], pruning_depth, &moves);

    let solver = Solver::new(moves, pruning_table);

    ida_star(cube, &solver, search_limit)
}