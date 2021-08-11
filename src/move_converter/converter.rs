use crate::generic_cube::{Move};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

pub fn from_scramble_string(scramble: String) -> Vec<Move> {
    scramble.split(" ").map(convert_move).collect()
}

fn convert_move(mv: &str) -> Move {
    match mv {
        "U" => U(Standard),
        "R" => R(Standard),
        "F" => F(Standard),
        "L" => L(Standard),
        "D" => D(Standard),
        "B" => B(Standard),
        "X" => X(Standard),
        "Y" => Y(Standard),
        "Z" => Z(Standard),
        "U'" => U(Inverse),
        "R'" => R(Inverse),
        "F'" => F(Inverse),
        "L'" => L(Inverse),
        "D'" => D(Inverse),
        "B'" => B(Inverse),
        "X'" => X(Inverse),
        "Y'" => Y(Inverse),
        "Z'" => Z(Inverse),
        "U2" => U(Double),
        "R2" => R(Double),
        "F2" => F(Double),
        "L2" => L(Double),
        "D2" => D(Double),
        "B2" => B(Double),
        "X2" => X(Double),
        "Y2" => Y(Double),
        "Z2" => Z(Double),
        _ => panic!()
    }
}