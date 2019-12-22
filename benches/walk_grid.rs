
extern crate aicup2019;

use criterion::{Criterion, criterion_group, criterion_main};
use aicup2019::my_strategy::{Vec2, WalkGrid};

fn walk_grid_next(c: &mut Criterion) {
    c.bench_function("walk_grid_next", |b| {
        b.iter(move || {
            WalkGrid::new(Vec2::new(23.123, 13.62), Vec2::new(8.954, 35.1)).count()
        })
    });
}

criterion_group!(benches, walk_grid_next);
criterion_main!(benches);
