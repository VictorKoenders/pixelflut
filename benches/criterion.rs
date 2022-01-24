mod benchmarks;

use criterion::{criterion_group, criterion_main};

criterion_group!(pixelflut, benchmarks::parsing_coordinates::bench);
criterion_main!(pixelflut);
