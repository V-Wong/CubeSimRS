use crate::generic_cube::{Move};

pub fn from_scramble_string(scramble: String) -> Vec<Move> {
    scramble.split(" ").map(convert_move).collect()
}

fn convert_move(mv: &str) -> Move {
    match mv {
        "U" => Move::U,
        "R" => Move::R,
        "F" => Move::F,
        "L" => Move::L,
        "D" => Move::D,
        "B" => Move::B,
        "X" => Move::X,
        "Y" => Move::Y,
        "Z" => Move::Z,
        "U'" => Move::U_,
        "R'" => Move::R_,
        "F'" => Move::F_,
        "L'" => Move::L_,
        "D'" => Move::D_,
        "B'" => Move::B_,
        "X'" => Move::X_,
        "Y'" => Move::Y_,
        "Z'" => Move::Z_,
        "U2" => Move::U2,
        "R2" => Move::R2,
        "F2" => Move::F2,
        "L2" => Move::L2,
        "D2" => Move::D2,
        "B2" => Move::B2,
        "X2" => Move::X2,
        "Y2" => Move::Y2,
        "Z2" => Move::Z2,
        _ => panic!()
    }
}