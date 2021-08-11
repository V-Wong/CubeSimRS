//
// Move Converter Tests
//

use cubesim::generic_cube::{Move, MoveVariant};
use cubesim::move_converter::{from_scramble_string};

#[test]
fn test_basic_moves() {
    assert_eq!(from_scramble_string(String::from("U"))[0], Move::U(MoveVariant::Standard));
    assert_eq!(from_scramble_string(String::from("F"))[0], Move::F(MoveVariant::Standard));
    assert_eq!(from_scramble_string(String::from("R"))[0], Move::R(MoveVariant::Standard));
    assert_eq!(from_scramble_string(String::from("D"))[0], Move::D(MoveVariant::Standard));
    assert_eq!(from_scramble_string(String::from("L"))[0], Move::L(MoveVariant::Standard));
    assert_eq!(from_scramble_string(String::from("B"))[0], Move::B(MoveVariant::Standard));

    assert_eq!(from_scramble_string(String::from("X"))[0], Move::X(MoveVariant::Standard));
    assert_eq!(from_scramble_string(String::from("Y"))[0], Move::Y(MoveVariant::Standard));
    assert_eq!(from_scramble_string(String::from("Z"))[0], Move::Z(MoveVariant::Standard));
}

#[test]
fn test_double_moves() {
    assert_eq!(from_scramble_string(String::from("U2"))[0], Move::U(MoveVariant::Double));
    assert_eq!(from_scramble_string(String::from("F2"))[0], Move::F(MoveVariant::Double));
    assert_eq!(from_scramble_string(String::from("R2"))[0], Move::R(MoveVariant::Double));
    assert_eq!(from_scramble_string(String::from("D2"))[0], Move::D(MoveVariant::Double));
    assert_eq!(from_scramble_string(String::from("L2"))[0], Move::L(MoveVariant::Double));
    assert_eq!(from_scramble_string(String::from("B2"))[0], Move::B(MoveVariant::Double));

    assert_eq!(from_scramble_string(String::from("X2"))[0], Move::X(MoveVariant::Double));
    assert_eq!(from_scramble_string(String::from("Y2"))[0], Move::Y(MoveVariant::Double));
    assert_eq!(from_scramble_string(String::from("Z2"))[0], Move::Z(MoveVariant::Double));
}

#[test]
fn test_inverse_moves() {
    assert_eq!(from_scramble_string(String::from("U'"))[0], Move::U(MoveVariant::Inverse));
    assert_eq!(from_scramble_string(String::from("F'"))[0], Move::F(MoveVariant::Inverse));
    assert_eq!(from_scramble_string(String::from("R'"))[0], Move::R(MoveVariant::Inverse));
    assert_eq!(from_scramble_string(String::from("D'"))[0], Move::D(MoveVariant::Inverse));
    assert_eq!(from_scramble_string(String::from("L'"))[0], Move::L(MoveVariant::Inverse));
    assert_eq!(from_scramble_string(String::from("B'"))[0], Move::B(MoveVariant::Inverse));

    assert_eq!(from_scramble_string(String::from("X'"))[0], Move::X(MoveVariant::Inverse));
    assert_eq!(from_scramble_string(String::from("Y'"))[0], Move::Y(MoveVariant::Inverse));
    assert_eq!(from_scramble_string(String::from("Z'"))[0], Move::Z(MoveVariant::Inverse));
}

#[test]
fn test_long_scramble() {
    assert_eq!(from_scramble_string(String::from("R U R' U' R' F R2 U' R' U' R U R' F'")), vec![
        Move::R(MoveVariant::Standard),
        Move::U(MoveVariant::Standard),
        Move::R(MoveVariant::Inverse),
        Move::U(MoveVariant::Inverse),
        Move::R(MoveVariant::Inverse),
        Move::F(MoveVariant::Standard),
        Move::R(MoveVariant::Double),
        Move::U(MoveVariant::Inverse),
        Move::R(MoveVariant::Inverse),
        Move::U(MoveVariant::Inverse),
        Move::R(MoveVariant::Standard),
        Move::U(MoveVariant::Standard),
        Move::R(MoveVariant::Inverse),
        Move::F(MoveVariant::Inverse),
    ]);
}