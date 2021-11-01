use crate::generic_cube::{Move, MoveVariant};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

/// Converts a WCA Notation scramble into ``Vec<Move>``.
pub fn parse_scramble(scramble: String) -> Vec<Move> {
    scramble.split(' ').map(convert_move).collect()
}

fn convert_move(mv: &str) -> Move {
    let slice = get_slice(mv);
    let variant = get_variant(mv);

    if !mv.contains("w") {
        match &mv[0..1] {
            "U" => U(variant),
            "R" => R(variant),
            "F" => F(variant),
            "L" => L(variant),
            "D" => D(variant),
            "B" => B(variant),
            "x" => X(variant),
            "y" => Y(variant),
            "z" => Z(variant),
            _ => panic!()
        }    
    } else if mv.contains("U") {
        Uw(slice, variant)
    } else if mv.contains("R") {
        Rw(slice, variant)
    } else if mv.contains("F") {
        Fw(slice, variant)
    } else if mv.contains("L") {
        Lw(slice, variant)
    } else if mv.contains("D") {
        Dw(slice, variant)
    } else if mv.contains("B") {
        Bw(slice, variant)
    } else if mv.contains("x") {
        X(variant)
    } else if mv.contains("y") {
        Y(variant)
    } else if mv.contains("z") {
        Z(variant)
    } else {
        panic!()
    }
}

fn get_slice(mv: &str) -> i32 {
    if !mv.contains("w") {
        1
    } else {
        mv[0..1].parse::<i32>().unwrap_or(2)
    }
}

fn get_variant(mv: &str) -> MoveVariant {
    if mv.contains('2') {
        Double
    } else if mv.contains('\'') {
        Inverse
    } else {
        Standard
    }
}