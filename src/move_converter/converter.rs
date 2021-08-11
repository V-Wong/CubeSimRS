use crate::generic_cube::{Move, MoveVariant};

pub fn from_scramble_string(scramble: String) -> Vec<Move> {
    scramble.split(" ").map(convert_move).collect()
}

fn convert_move(mv: &str) -> Move {
    match mv {
        "U" => Move::U(MoveVariant::Standard),
        "R" => Move::R(MoveVariant::Standard),
        "F" => Move::F(MoveVariant::Standard),
        "L" => Move::L(MoveVariant::Standard),
        "D" => Move::D(MoveVariant::Standard),
        "B" => Move::B(MoveVariant::Standard),
        "X" => Move::X(MoveVariant::Standard),
        "Y" => Move::Y(MoveVariant::Standard),
        "Z" => Move::Z(MoveVariant::Standard),
        "U'" => Move::U(MoveVariant::Inverse),
        "R'" => Move::R(MoveVariant::Inverse),
        "F'" => Move::F(MoveVariant::Inverse),
        "L'" => Move::L(MoveVariant::Inverse),
        "D'" => Move::D(MoveVariant::Inverse),
        "B'" => Move::B(MoveVariant::Inverse),
        "X'" => Move::X(MoveVariant::Inverse),
        "Y'" => Move::Y(MoveVariant::Inverse),
        "Z'" => Move::Z(MoveVariant::Inverse),
        "U2" => Move::U(MoveVariant::Double),
        "R2" => Move::R(MoveVariant::Double),
        "F2" => Move::F(MoveVariant::Double),
        "L2" => Move::L(MoveVariant::Double),
        "D2" => Move::D(MoveVariant::Double),
        "B2" => Move::B(MoveVariant::Double),
        "X2" => Move::X(MoveVariant::Double),
        "Y2" => Move::Y(MoveVariant::Double),
        "Z2" => Move::Z(MoveVariant::Double),
        _ => panic!()
    }
}