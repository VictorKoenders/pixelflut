use criterion::Criterion;
use pixelflut::parse::*;
use rand::{thread_rng, Rng};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("color");

    const ENTRIES: usize = 1_000_000;
    let mut input = String::with_capacity(ENTRIES * 6);
    let mut expected = Vec::with_capacity(ENTRIES);
    let mut rng = thread_rng();
    for _ in 0..ENTRIES {
        let mut gen_color = || {
            let val: u8 = rng.gen();
            (
                val,
                if rng.gen::<bool>() {
                    format!("{:02x}", val)
                } else {
                    format!("{:02X}", val)
                },
            )
        };

        let (r, r_str) = gen_color();
        let (g, g_str) = gen_color();
        let (b, b_str) = gen_color();

        input += &r_str;
        input += &g_str;
        input += &b_str;

        expected.push(((r, g, b), format!("{}{}{}", r_str, g_str, b_str)));
    }

    #[cfg(feature = "memory-cache")]
    {
        let mut memcache = memcache::HexCache::new();
        memcache.init();
        group.bench_with_input(
            "memcache",
            &(&memcache, input.as_bytes(), &expected),
            |b, &(cache, input, expected)| {
                b.iter(|| {
                    let mut buffer = input;

                    let mut idx = 0;

                    while let Some(output) = cache.parse_color(buffer) {
                        assert_eq!(
                            output,
                            expected[idx].0,
                            "Expected {:?}, got {:?}, input: {:?}", expected[idx].0, output, expected[idx].1
                        );
                        idx += 1;
                        buffer = &buffer[6..];
                    }
                    assert_eq!(idx, expected.len());
                    assert!(buffer.is_empty(), "Remaining bytes: {:?}", buffer);
                });
            },
        );
    }

    group.bench_with_input(
        "bytewise",
        &(input.as_bytes(), &expected),
        |b, &(input, expected)| {
            b.iter(|| {
                let mut buffer = input;
                let mut idx = 0;
                while let Some(output) = bytewise::parse_color(buffer) {
                    assert_eq!(
                        output,
                        expected[idx].0,
                        "Expected {:?}, got {:?}, input: {:?}", expected[idx].0, output, expected[idx].1
                    );
                    idx += 1;
                    buffer = &buffer[6..];
                }
                assert_eq!(idx, expected.len());
                assert!(buffer.is_empty(), "Remaining bytes: {:?}", buffer);
            });
        },
    );

    group.bench_with_input(
        "bytewise_unwrapped",
        &(input.as_bytes(), &expected),
        |b, &(input, expected)| {
            b.iter(|| {
                let mut buffer = input;
                let mut idx = 0;
                while let Some(output) = bytewise::parse_color_unwrapped(buffer) {
                    assert_eq!(
                        output,
                        expected[idx].0,
                        "Expected {:?}, got {:?}, input: {:?}", expected[idx].0, output, expected[idx].1
                    );
                    idx += 1;
                    buffer = &buffer[6..];
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
                while let Some(output) = std::parse_color(buffer) {
                    assert_eq!(
                        output,
                        expected[idx].0,
                        "Expected {:?}, got {:?}, input: {:?}", expected[idx].0, output, expected[idx].1
                    );
                    idx += 1;
                    buffer = &buffer[6..];
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
                while let Some(output) = python_generated::parse_color(buffer) {
                    assert_eq!(
                        output,
                        expected[idx].0,
                        "Expected {:?}, got {:?}, input: {:?}", expected[idx].0, output, expected[idx].1
                    );
                    idx += 1;
                    buffer = &buffer[6..];
                }
                assert_eq!(idx, expected.len());
                assert!(buffer.is_empty(), "Remaining bytes: {:?}", buffer);
            });
        },
    );

    group.finish();
}
