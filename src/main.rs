mod generic_cube;
mod geometric_cube;
mod facelet_cube;
mod generic_solver;
mod thistlethwaite;
mod scramble_parser;

use generic_cube::{Cube, Move, MoveVariant};
use facelet_cube::FaceletCube;
use thistlethwaite::solve;
use scramble_parser::parse_scramble;

fn main() {
    let cube = &FaceletCube::new(3).apply_moves(&parse_scramble(String::from("U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2")));

    let solution = solve(cube);

    if let Some(s) = solution {
        println!("{:?}", cube.apply_moves(&s).is_solved());
    }
}
