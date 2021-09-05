use crate::generic_cube::{Move, MoveVariant};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

pub fn from_scramble_string(scramble: String) -> Vec<Move> {
    scramble.split(' ').map(convert_move).collect()
}

fn convert_move(mv: &str) -> Move {
    let variant = get_variant(mv);

    match &mv[0..1] {
        "U" => U(variant),
        "R" => R(variant),
        "F" => F(variant),
        "L" => L(variant),
        "D" => D(variant),
        "B" => B(variant),
        "X" => X(variant),
        "Y" => Y(variant),
        "Z" => Z(variant),
        _ => panic!()
    }
}

fn get_variant(mv: &str) -> MoveVariant {
    return if mv.contains('2') {
        Double
    } else if mv.contains('\'') {
        Inverse
    } else {
        Standard
    }
}