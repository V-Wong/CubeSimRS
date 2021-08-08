mod geometric_cube;

use geometric_cube::cube::{cube3, U_MOVE};

fn main() {
    println!("{}", cube3().is_solved());
    println!("{}", cube3().apply_move(U_MOVE).is_solved());
}
