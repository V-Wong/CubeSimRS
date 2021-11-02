use crate::generic_cube::{Move, MoveVariant};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

/// Converts a WCA Notation scramble into ``Vec<Move>``.
pub fn parse_scramble(scramble: String) -> Vec<Move> {
    scramble.trim().split_whitespace().map(convert_move).collect()
}

fn convert_move(mv: &str) -> Move {
    let slice = get_slice(mv);
    let variant = get_variant(mv);

    if !mv.contains('w') {
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
    } else if mv.contains('U') {
        Uw(slice, variant)
    } else if mv.contains('R') {
        Rw(slice, variant)
    } else if mv.contains('F') {
        Fw(slice, variant)
    } else if mv.contains('L') {
        Lw(slice, variant)
    } else if mv.contains('D') {
        Dw(slice, variant)
    } else if mv.contains('B') {
        Bw(slice, variant)
    } else if mv.contains('x') {
        X(variant)
    } else if mv.contains('y') {
        Y(variant)
    } else if mv.contains('z') {
        Z(variant)
    } else {
        panic!()
    }
}

fn get_slice(mv: &str) -> i32 {
    if !mv.contains('w') {
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

/// Recursively merges adjacent moves of the same Move discriminant
/// until no further simplification is possible.
pub fn simplify_moves(moves: &[Move]) -> Vec<Move> {
    use std::mem::discriminant;
    let mut result = vec![];
    if moves.is_empty() {
        return result;
    }

    // keep track of the current move and its amount of clockwise turns
    let mut curr: (Move, u8) = (moves[0], moves[0].get_variant() as u8);
    // pushes curr onto result if it has a non-zero number of turns
    let mut push_curr = |current: (Move, u8)| {
        if current.1 != 0 {
            let variant = match current.1 {
                1 => MoveVariant::Standard,
                2 => MoveVariant::Double,
                3 => MoveVariant::Inverse,
                _ => MoveVariant::Standard
            };
            result.push(current.0.with_variant(variant));
        }
    };

    // merge adjacent moves of the same type
    for mv in moves[1..].iter() {
        if discriminant(&curr.0) == discriminant(mv) {
            curr.1 += mv.get_variant() as u8;
            curr.1 %= 4;
        } else {
            push_curr(curr);
            curr = (*mv, mv.get_variant() as u8);
        }
    }
    push_curr(curr);

    // if moves couldn't be simplified further
    if result.len() == moves.len() {
        return result
    }
    simplify_moves(result.as_slice())
}