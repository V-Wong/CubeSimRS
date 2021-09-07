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

#[doc(hidden)]
pub mod prelude;
#[doc(hidden)]
pub mod cube_implementors {
    pub use crate::facelet_cube::FaceletCube;
    pub use crate::geometric_cube::GeoCube;
}

pub use scramble_parser::parse_scramble;
pub use generic_cube::{Cube, Face, Move, MoveVariant};
pub use cube_implementors::{FaceletCube, GeoCube};

mod generic_cube;
mod generic_solver;
mod facelet_cube;
mod geometric_cube;
mod scramble_parser;