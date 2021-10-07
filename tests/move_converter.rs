//
// Move Parser Tests
//

use cubesim::parse_scramble;
use cubesim::prelude::{Move::*, MoveVariant::*};

#[test]
fn test_basic_moves() {
    assert_eq!(parse_scramble(String::from("U"))[0], U(Standard));
    assert_eq!(parse_scramble(String::from("F"))[0], F(Standard));
    assert_eq!(parse_scramble(String::from("R"))[0], R(Standard));
    assert_eq!(parse_scramble(String::from("D"))[0], D(Standard));
    assert_eq!(parse_scramble(String::from("L"))[0], L(Standard));
    assert_eq!(parse_scramble(String::from("B"))[0], B(Standard));

    assert_eq!(parse_scramble(String::from("X"))[0], X(Standard));
    assert_eq!(parse_scramble(String::from("Y"))[0], Y(Standard));
    assert_eq!(parse_scramble(String::from("Z"))[0], Z(Standard));
}

#[test]
fn test_double_moves() {
    assert_eq!(parse_scramble(String::from("U2"))[0], U(Double));
    assert_eq!(parse_scramble(String::from("F2"))[0], F(Double));
    assert_eq!(parse_scramble(String::from("R2"))[0], R(Double));
    assert_eq!(parse_scramble(String::from("D2"))[0], D(Double));
    assert_eq!(parse_scramble(String::from("L2"))[0], L(Double));
    assert_eq!(parse_scramble(String::from("B2"))[0], B(Double));

    assert_eq!(parse_scramble(String::from("X2"))[0], X(Double));
    assert_eq!(parse_scramble(String::from("Y2"))[0], Y(Double));
    assert_eq!(parse_scramble(String::from("Z2"))[0], Z(Double));
}

#[test]
fn test_inverse_moves() {
    assert_eq!(parse_scramble(String::from("U'"))[0], U(Inverse));
    assert_eq!(parse_scramble(String::from("F'"))[0], F(Inverse));
    assert_eq!(parse_scramble(String::from("R'"))[0], R(Inverse));
    assert_eq!(parse_scramble(String::from("D'"))[0], D(Inverse));
    assert_eq!(parse_scramble(String::from("L'"))[0], L(Inverse));
    assert_eq!(parse_scramble(String::from("B'"))[0], B(Inverse));

    assert_eq!(parse_scramble(String::from("X'"))[0], X(Inverse));
    assert_eq!(parse_scramble(String::from("Y'"))[0], Y(Inverse));
    assert_eq!(parse_scramble(String::from("Z'"))[0], Z(Inverse));
}

#[test]
fn test_long_scramble() {
    assert_eq!(parse_scramble(String::from("R U R' U' R' F R2 U' R' U' R U R' F'")), vec![
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

#[test]
fn test_wide_moves() {
    assert_eq!(parse_scramble(String::from("Rw 3Fw 5Bw' 3Lw2")), vec![
        Rw(2, Standard),
        Fw(3, Standard),
        Bw(5, Inverse),
        Lw(3, Double)
    ]);
}