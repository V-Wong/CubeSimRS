mod geometric_cube;
mod generic_cube;

use generic_cube::{Cube, Move};
use geometric_cube::{cube3};

fn main() {
    println!("{}", cube3().is_solved());
    println!("{}", cube3().apply_move(Move::U)
                          .apply_move(Move::U)
                          .apply_move(Move::U)
                          .is_solved()
    );
}
