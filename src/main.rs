mod geometric_cube;

use geometric_cube::cube::{cube3};
use geometric_cube::moves;

fn main() {
    println!("{}", cube3().is_solved());
    println!("{}", cube3().apply_move(moves::U_MOVE)
                          .apply_move(moves::U_MOVE)
                          .apply_move(moves::U_MOVE)
                          .apply_move(moves::U_MOVE)
                          .is_solved()
    );
}
