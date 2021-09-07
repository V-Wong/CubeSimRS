//! A simple Rubik's Cube simulator and solver.
//! 
//! # Quick Start Guide
//! To quickly get a Rubik's Cube simulation running, we can follow
//! the code sample below:
//! 
//! ```rust
//! use cubesim::{Cube, FaceletCube, Move, MoveVariant};
//! 
//! let cube = FaceletCube::new(3);
//! let rotated_cube = cube.apply_move(Move::U(MoveVariant::Double));
//! println!("{}", rotated_cube.get_state());
//! ```

pub mod prelude;
pub mod cube_implementors {
    pub use crate::facelet_cube::FaceletCube;
    pub use crate::geometric_cube::GeoCube;
}
pub use scramble_parser::parse_scramble;

mod generic_cube;
mod generic_solver;
mod facelet_cube;
mod geometric_cube;
mod scramble_parser;