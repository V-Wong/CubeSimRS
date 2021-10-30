//! A simple Rubik's Cube simulator (and eventually solver).
//! 
//! # Quick Start Guide
//! 
//! To quickly get a Rubik's Cube simulation running, we can follow the code sample below:
//! 
//! ```rust
//! use cubesim::prelude::*;
//! use cubesim::cube_implementors::FaceletCube;
//! 
//! let cube = FaceletCube::new(3);
//! let turned_cube = cube.apply_move(Move::U(MoveVariant::Double));
//! println!("{:?}", turned_cube.get_state());
//! ```
//! 
//! # Cube Trait Overview
//! To support multiple underlying implementations of a Rubik's Cube, we define a ``Cube`` trait
//! which define the minimal set of behaviours expected of a Rubik's Cube. Specific implementations 
//! can then be used for different scenarios that fit its characteristics. For example, the ``FaceletCube``
//! is the most performant and versatile while the ``GeoCube`` allows for easy 3D modelling.

#[doc(hidden)]
pub mod prelude;
#[doc(hidden)]
pub mod cube_implementors {
    pub use crate::facelet_cube::FaceletCube;
    pub use crate::geometric_cube::GeoCube;
}
pub mod solvers {
    pub use crate::thistlethwaite::solve;
}

pub use scramble_parser::parse_scramble;
pub use generic_cube::{Cube, Face, Move, MoveVariant, solved_state, all_moves};
pub use cube_implementors::{FaceletCube, GeoCube};

mod generic_cube;
mod generic_solver;
mod facelet_cube;
mod geometric_cube;
mod scramble_parser;
mod thistlethwaite;