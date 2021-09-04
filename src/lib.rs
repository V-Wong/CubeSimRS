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

mod generic_cube;
mod geometric_cube;
mod facelet_cube;
pub mod move_converter;

pub use {
    generic_cube::{Cube, Move, MoveVariant, Face},
    geometric_cube::GeoCube, 
    facelet_cube::FaceletCube
};