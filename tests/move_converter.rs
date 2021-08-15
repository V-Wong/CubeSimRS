//
// Move Converter Tests
//

use cubesim::move_converter::{from_scramble_string};
use cubesim::Move::*;
use cubesim::MoveVariant::*;

#[test]
fn test_basic_moves() {
    assert_eq!(from_scramble_string(String::from("U"))[0], U(Standard));
    assert_eq!(from_scramble_string(String::from("F"))[0], F(Standard));
    assert_eq!(from_scramble_string(String::from("R"))[0], R(Standard));
    assert_eq!(from_scramble_string(String::from("D"))[0], D(Standard));
    assert_eq!(from_scramble_string(String::from("L"))[0], L(Standard));
    assert_eq!(from_scramble_string(String::from("B"))[0], B(Standard));

    assert_eq!(from_scramble_string(String::from("X"))[0], X(Standard));
    assert_eq!(from_scramble_string(String::from("Y"))[0], Y(Standard));
    assert_eq!(from_scramble_string(String::from("Z"))[0], Z(Standard));
}

#[test]
fn test_double_moves() {
    assert_eq!(from_scramble_string(String::from("U2"))[0], U(Double));
    assert_eq!(from_scramble_string(String::from("F2"))[0], F(Double));
    assert_eq!(from_scramble_string(String::from("R2"))[0], R(Double));
    assert_eq!(from_scramble_string(String::from("D2"))[0], D(Double));
    assert_eq!(from_scramble_string(String::from("L2"))[0], L(Double));
    assert_eq!(from_scramble_string(String::from("B2"))[0], B(Double));

    assert_eq!(from_scramble_string(String::from("X2"))[0], X(Double));
    assert_eq!(from_scramble_string(String::from("Y2"))[0], Y(Double));
    assert_eq!(from_scramble_string(String::from("Z2"))[0], Z(Double));
}

#[test]
fn test_inverse_moves() {
    assert_eq!(from_scramble_string(String::from("U'"))[0], U(Inverse));
    assert_eq!(from_scramble_string(String::from("F'"))[0], F(Inverse));
    assert_eq!(from_scramble_string(String::from("R'"))[0], R(Inverse));
    assert_eq!(from_scramble_string(String::from("D'"))[0], D(Inverse));
    assert_eq!(from_scramble_string(String::from("L'"))[0], L(Inverse));
    assert_eq!(from_scramble_string(String::from("B'"))[0], B(Inverse));

    assert_eq!(from_scramble_string(String::from("X'"))[0], X(Inverse));
    assert_eq!(from_scramble_string(String::from("Y'"))[0], Y(Inverse));
    assert_eq!(from_scramble_string(String::from("Z'"))[0], Z(Inverse));
}

#[test]
fn test_long_scramble() {
    assert_eq!(from_scramble_string(String::from("R U R' U' R' F R2 U' R' U' R U R' F'")), vec![
        R(Standard),
        U(Standard),
        R(Inverse),
        U(Inverse),
        R(Inverse),
        F(Standard),
        R(Double),
        U(Inverse),
        R(Inverse),
        U(Inverse),
        R(Standard),
        U(Standard),
        R(Inverse),
        F(Inverse),
    ]);
}