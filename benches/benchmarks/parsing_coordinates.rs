use criterion::Criterion;
use pixelflut::parse::*;
use rand::{thread_rng, Rng};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing_coordinates");

    const ENTRIES: usize = 1_000_000;
    let mut input = String::with_capacity(ENTRIES * 5);
    let mut expected = Vec::with_capacity(ENTRIES);
    let mut rng = thread_rng();
    for _ in 0..ENTRIES {
        let num = rng.gen_range(0..=MAX_VALID_NUMBER);
        input += &num.to_string();
        input.push(' ');
        expected.push(num);
    }

    group.bench_with_input(
        "bytewise",
        &(input.as_bytes(), &expected),
        |b, &(input, expected)| {
            b.iter(|| {
                let mut buffer = input;
                let mut idx = 0;
                while let Some((num, remaining)) = bytewise::parse_coordinate(buffer) {
                    assert_eq!(num, expected[idx]);
                    idx += 1;
                    buffer = remaining;
                }
                assert_eq!(idx, expected.len());
                assert!(buffer.is_empty(), "Remaining bytes: {:?}", buffer);
            });
        },
    );

    group.bench_with_input(
        "std",
        &(input.as_bytes(), &expected),
        |b, &(input, expected)| {
            b.iter(|| {
                let mut buffer = input;
                let mut idx = 0;
                while let Some((num, remaining)) = std::parse_coordinate(buffer) {
                    assert_eq!(num, expected[idx]);
                    idx += 1;
                    buffer = remaining;
                }
                assert_eq!(idx, expected.len());
                assert!(buffer.is_empty(), "Remaining bytes: {:?}", buffer);
            });
        },
    );

    group.bench_with_input(
        "python generated",
        &(input.as_bytes(), &expected),
        |b, &(input, expected)| {
            b.iter(|| {
                let mut buffer = input;
                let mut idx = 0;
                while let Some((num, remaining)) = python_generated::parse_coordinate(buffer) {
                    assert_eq!(num, expected[idx]);
                    idx += 1;
                    buffer = remaining;
                }
                assert_eq!(idx, expected.len());
                assert!(buffer.is_empty(), "Remaining bytes: {:?}", buffer);
            });
        },
    );

    group.finish();
}
