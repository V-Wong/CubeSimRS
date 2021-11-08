//
// Thistlethwaite Solver Benchmarks
//

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cubesim::prelude::*;
use cubesim::cube_implementors::{FaceletCube};
use cubesim::solve;
use cubesim::parse_scramble;

pub fn superflip(c: &mut Criterion) {
    let scramble = parse_scramble(String::from("U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2"));
    let cube = FaceletCube::new(3).apply_moves(&scramble);

    c.bench_function("Superflip", |b| b.iter(|| {
        black_box(solve(&cube));
    }));
}

pub fn random_scramble(c: &mut Criterion) {
    let scramble = parse_scramble(String::from("D2 U2 R' U2 B2 L U2 L' D2 L' R' U2 F' L' B L B2 U' R"));
    let cube = FaceletCube::new(3).apply_moves(&scramble);

    c.bench_function("Random scramble", |b| b.iter(|| {
        black_box(solve(&cube));
    }));
}

criterion_group!(benches, superflip, random_scramble);
criterion_main!(benches);