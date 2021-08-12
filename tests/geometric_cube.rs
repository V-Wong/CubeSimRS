//
// Geometric Cube Tests
//

use cubesim::generic_cube::{Cube, Move, MoveVariant};
use cubesim::generic_cube::Face::*;
use cubesim::geometric_cube::{cube3};

//
// State Tests
//
#[test]
fn test_solved_state() {
    println!("{:?}", cube3().get_state());

    assert!(cube3().get_state() == vec![
        U, U, U, U, U, U, U, U, U,
        R, R, R, R, R, R, R, R, R,
        F, F, F, F, F, F, F, F, F,
        D, D, D, D, D, D, D, D, D,
        L, L, L, L, L, L, L, L, L,
        B, B, B, B, B, B, B, B, B
    ]);
}

#[test]
fn test_u_move() {
    println!("{:?}", cube3().apply_move(Move::U(MoveVariant::Standard)).get_state());

    assert!(cube3().apply_move(Move::U(MoveVariant::Standard)).get_state() == vec![
        U, U, U, U, U, U, U, U, U,
        B, B, B, R, R, R, R, R, R,
        R, R, R, F, F, F, F, F, F,
        D, D, D, D, D, D, D, D, D,
        F, F, F, L, L, L, L, L, L,
        L, L, L, B, B, B, B, B, B
    ]);
}

#[test]
fn test_r_move() {
    assert!(cube3().apply_move(Move::R(MoveVariant::Standard)).get_state() == vec![
        U, U, F, U, U, F, U, U, F,
        R, R, R, R, R, R, R, R, R,
        F, F, D, F, F, D, F, F, D,
        D, D, B, D, D, B, D, D, B,
        L, L, L, L, L, L, L, L, L,
        U, B, B, U, B, B, U, B, B
    ]);
}

#[test]
fn test_f_move() {
    assert!(cube3().apply_move(Move::F(MoveVariant::Standard)).get_state() == vec![
        U, U, U, U, U, U, L, L, L,
        U, R, R, U, R, R, U, R, R,
        F, F, F, F, F, F, F, F, F,
        R, R, R, D, D, D, D, D, D,
        L, L, D, L, L, D, L, L, D,
        B, B, B, B, B, B, B, B, B
    ]);
}

#[test]
fn test_l_move() {
    assert!(cube3().apply_move(Move::L(MoveVariant::Standard)).get_state() == vec![
        B, U, U, B, U, U, B, U, U,
        R, R, R, R, R, R, R, R, R,
        U, F, F, U, F, F, U, F, F,
        F, D, D, F, D, D, F, D, D,
        L, L, L, L, L, L, L, L, L,
        B, B, D, B, B, D, B, B, D
    ]);
}

#[test]
fn test_d_move() {
    assert!(cube3().apply_move(Move::D(MoveVariant::Standard)).get_state() == vec![
        U, U, U, U, U, U, U, U, U,
        R, R, R, R, R, R, F, F, F,
        F, F, F, F, F, F, L, L, L,
        D, D, D, D, D, D, D, D, D,
        L, L, L, L, L, L, B, B, B,
        B, B, B, B, B, B, R, R, R
    ]);
}

#[test]
fn test_b_move() {
    assert!(cube3().apply_move(Move::B(MoveVariant::Standard)).get_state() == vec![
        R, R, R, U, U, U, U, U, U,
        R, R, D, R, R, D, R, R, D,
        F, F, F, F, F, F, F, F, F,
        D, D, D, D, D, D, L, L, L,
        U, L, L, U, L, L, U, L, L,
        B, B, B, B, B, B, B, B, B
    ]);
}