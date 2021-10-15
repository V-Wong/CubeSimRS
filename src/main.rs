mod generic_cube;
mod geometric_cube;
mod facelet_cube;
mod iddfs;

use generic_cube::{Cube, Move, MoveVariant, all_moves, solved_state};
use generic_cube::Move::*;
use generic_cube::MoveVariant::*;
use facelet_cube::FaceletCube;
use iddfs::solve;

fn main() {
    println!("{:?}", solve(
        &FaceletCube::new(3).apply_moves(&vec![U(Standard), D(Inverse), F(Double), R(Double), B(Double), F(Double), L(Inverse)]), 
        &all_moves(3),
        &solved_state(3),
        7
    ))
}
