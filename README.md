# CubeSim V2
CubeSim is a Rubik's Cube simulator and solver written entirely in Rust. 

## Features
- Simulation of a 3x3x3 Rubik's Cube.

## Planned Features
- Optimal solutions using variations of [IDA*](https://en.wikipedia.org/wiki/Iterative_deepening_A*).
- Generalisation to NxNxN cubes.
- User interface (web interface using WASM perhaps?).

## Usage
Build the project (optionally in release mode):
```sh
$ cargo build [--release]
```

Run the project:
```sh
$ cargo run
```

## Project Structure
- ``src/generic_cube``: This contains a generic definition of a Cube that all concrete implementations must adhere to. This includes a **Cube Trait**, a **Move enum** and a **Face enum**.
- ``src/geometric_cube``: This is a geometric implementation of a Cube which represents stickers of a cube as **3-dimensional vectors** and moves as **rotation matrices**. This implementation is highly inefficient but it provides a means to bootstrap more efficient implementations.
- ``src/facelet_cube``: This is a facelet implementation of a cube which represents stickers of a cube with an **identifier in an 1-dimensional array** and moves as **index to index mappings**. This implementation is efficient and is implemented by converting instances of the geometric cube moves.

## Resources Used
- [Onionhoney's extremely well written article on modelling Rubik's Cubes](https://observablehq.com/@onionhoney/how-to-model-a-rubiks-cube)
- [My original suboptimal implementation of a Rubik's Cube simulator and solver](https://github.com/V-Wong/CubeSim)
