//
// Geometric Cube Benchmarks
//

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cubesim::prelude::*;
use cubesim::cube_implementors::{GeoCube};

pub fn single_moves(c: &mut Criterion) {
    c.bench_function("All single moves", |b| b.iter(|| {
        black_box(GeoCube::new(3).apply_move(Move::U(MoveVariant::Standard))
                                 .apply_move(Move::R(MoveVariant::Standard))
                                 .apply_move(Move::F(MoveVariant::Standard))
                                 .apply_move(Move::L(MoveVariant::Standard))
                                 .apply_move(Move::D(MoveVariant::Standard))
                                 .apply_move(Move::B(MoveVariant::Standard))
        );
    }));
}

pub fn state(c: &mut Criterion) {
    c.bench_function("Obtaining state", |b| b.iter(|| {
        black_box(GeoCube::new(3).get_state());
    }));
}

criterion_group!(benches, single_moves, state);
criterion_main!(benches);