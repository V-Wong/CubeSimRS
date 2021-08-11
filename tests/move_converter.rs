//
// Move Converter Tests
//

use cubesim::generic_cube::{Move};
use cubesim::move_converter::{from_scramble_string};

#[test]
fn test_basic_moves() {
    assert_eq!(from_scramble_string(String::from("U"))[0], Move::U);
    assert_eq!(from_scramble_string(String::from("F"))[0], Move::F);
    assert_eq!(from_scramble_string(String::from("R"))[0], Move::R);
    assert_eq!(from_scramble_string(String::from("D"))[0], Move::D);
    assert_eq!(from_scramble_string(String::from("L"))[0], Move::L);
    assert_eq!(from_scramble_string(String::from("B"))[0], Move::B);

    assert_eq!(from_scramble_string(String::from("X"))[0], Move::X);
    assert_eq!(from_scramble_string(String::from("Y"))[0], Move::Y);
    assert_eq!(from_scramble_string(String::from("Z"))[0], Move::Z);
}

#[test]
fn test_double_moves() {
    assert_eq!(from_scramble_string(String::from("U2"))[0], Move::U2);
    assert_eq!(from_scramble_string(String::from("F2"))[0], Move::F2);
    assert_eq!(from_scramble_string(String::from("R2"))[0], Move::R2);
    assert_eq!(from_scramble_string(String::from("D2"))[0], Move::D2);
    assert_eq!(from_scramble_string(String::from("L2"))[0], Move::L2);
    assert_eq!(from_scramble_string(String::from("B2"))[0], Move::B2);

    assert_eq!(from_scramble_string(String::from("X2"))[0], Move::X2);
    assert_eq!(from_scramble_string(String::from("Y2"))[0], Move::Y2);
    assert_eq!(from_scramble_string(String::from("Z2"))[0], Move::Z2);
}

#[test]
fn test_inverse_moves() {
    assert_eq!(from_scramble_string(String::from("U'"))[0], Move::U_);
    assert_eq!(from_scramble_string(String::from("F'"))[0], Move::F_);
    assert_eq!(from_scramble_string(String::from("R'"))[0], Move::R_);
    assert_eq!(from_scramble_string(String::from("D'"))[0], Move::D_);
    assert_eq!(from_scramble_string(String::from("L'"))[0], Move::L_);
    assert_eq!(from_scramble_string(String::from("B'"))[0], Move::B_);

    assert_eq!(from_scramble_string(String::from("X'"))[0], Move::X_);
    assert_eq!(from_scramble_string(String::from("Y'"))[0], Move::Y_);
    assert_eq!(from_scramble_string(String::from("Z'"))[0], Move::Z_);
}