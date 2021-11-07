# CubeSim V2
CubeSim is a Rubik's Cube simulator and solver written entirely in Rust.  

## Features
- Simulation of an arbitrarily sized NxNxN Rubik's Cube.
- Generation of solutions using the [Thistlethwaite algorithm](https://www.jaapsch.net/puzzles/thistle.htm)

## Planned Features
- User interface (web interface using WASM perhaps?).
- Optimal solutions using the [Kociemba algorithm](https://www.speedsolving.com/wiki/index.php/Kociemba's_Algorithm)

## Usage
The core types in the library are as follows:
- ``Cube trait``: To support multiple implementations of a Rubik’s Cube, we define a trait which includes the minimal set of behaviours expected of a Rubik’s Cube. Specific implementations can then be used for different scenarios. For example, the ``FaceletCube`` is most performant while the ``GeoCube`` allows for easy 3D modelling.
- ``Face enum``: A face of a Rubik’s Cube sticker represented in [WCA notation](https://www.worldcubeassociation.org/regulations/#article-12-notation).
- ``Move enum``: A move of a 3x3x3 Rubik’s Cube represented in [WCA notation](https://www.worldcubeassociation.org/regulations/#article-12-notation). Each ``Move`` must be tagged with a ``MoveVariant`` to completely define a move.
- ``MoveVariant enum``: A move variation that must be applied to the ``Move`` enum.

After understanding these core types, we can start writing a basic simulation:

```rs
use cubesim::prelude::{Cube, Face, Move, MoveVariant};
use cubesim::cube_implementors::FaceletCube;
 
let cube = FaceletCube::new(3);
let turned_cube = cube.apply_move(Move::U(MoveVariant::Double));
println!("{:?}", turned_cube.get_state());
```

To build more complex simulations and solvers, please follow our [official documentation](https://docs.rs/cubesim/0.0.4/cubesim/).

## Resources Used
- [Onionhoney's extremely well written article on modelling Rubik's Cubes](https://observablehq.com/@onionhoney/how-to-model-a-rubiks-cube)
- [My original suboptimal implementation of a Rubik's Cube simulator and solver](https://github.com/V-Wong/CubeSim)
