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
    type Movement = (Move, u8);
    let mut movement: Movement = (moves[0], moves[0].get_variant() as u8);

    // returns a Move if the simplified movement has any effect on a cube
    fn movement_to_move(m: Movement) -> Option<Move> {
        match m.1 % 4 {
            1 => Some(m.0.with_variant(MoveVariant::Standard)),
            2 => Some(m.0.with_variant(MoveVariant::Double)),
            3 => Some(m.0.with_variant(MoveVariant::Inverse)),
            _ => None,
        }
    }

    // merge adjacent moves of the same type
    for mv in moves[1..].iter() {
        if discriminant(&movement.0) == discriminant(mv) {
            movement.1 = (movement.1 + mv.get_variant() as u8) % 4;
        } else {
            if let Some(m) = movement_to_move(movement) { result.push(m) };
            movement = (*mv, mv.get_variant() as u8);
        }
    }
    if let Some(m) = movement_to_move(movement) { result.push(m) };

    // if moves couldn't be simplified further
    if result.len() == moves.len() { return result }
    simplify_moves(result.as_slice())
}