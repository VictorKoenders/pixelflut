mod benchmarks;

use criterion::{criterion_group, criterion_main};

criterion_main!(pixelflut);

criterion_group!(
    pixelflut,
    benchmarks::parsing_coordinates::bench,
    benchmarks::misc
);
