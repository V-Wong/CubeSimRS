mod generic_cube;
mod geometric_cube;
mod facelet_cube;

use generic_cube::{Cube, Move, MoveVariant};
use facelet_cube::FaceletCube;

fn main() {
    println!("{:?}", FaceletCube::new().apply_move(Move::U(MoveVariant::Standard)).get_state());
}
