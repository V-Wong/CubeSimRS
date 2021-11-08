//
// Facelet Cube Benchmarks
//

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cubesim::prelude::*;
use cubesim::{FaceletCube};

pub fn single_moves(c: &mut Criterion) {
    let cube = FaceletCube::new(3);

    c.bench_function("Facelet cube single moves", |b| b.iter(|| {
        black_box(cube.apply_move(Move::U(MoveVariant::Standard))
                      .apply_move(Move::R(MoveVariant::Standard))
                      .apply_move(Move::F(MoveVariant::Standard))
                      .apply_move(Move::L(MoveVariant::Standard))
                      .apply_move(Move::D(MoveVariant::Standard))
                      .apply_move(Move::B(MoveVariant::Standard))
        );
    }));
}

pub fn state(c: &mut Criterion) {
    let cube = FaceletCube::new(3);

    c.bench_function("Facelet cube state", |b| b.iter(|| {
        black_box(cube.get_state());
    }));
}

criterion_group!(benches, single_moves, state);
criterion_main!(benches);