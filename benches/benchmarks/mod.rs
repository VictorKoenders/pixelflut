use criterion::Criterion;

pub mod parsing_coordinates;

pub fn misc(_c: &mut Criterion) {
    #[cfg(feature = "memory-cache")]
    {
        use pixelflut::parse::*;
        let inputs: Vec<String> = (0..=MAX_VALID_NUMBER)
            .map(|n| n.to_string())
            .cycle()
            .take(1_000_000)
            .collect();

        let mut group = _c.benchmark_group("get_index_from_str");

        group.bench_with_input("v1", &inputs, |b, i| {
            b.iter(|| {
                i.iter()
                    .map(|s| memcache::get_index_from_str_v1(s.as_bytes()))
                    .collect::<Vec<_>>()
            })
        });

        group.bench_with_input("v2", &inputs, |b, i| {
            b.iter(|| {
                i.iter()
                    .map(|s| memcache::get_index_from_str_v2(s.as_bytes()))
                    .collect::<Vec<_>>()
            })
        });

        group.finish();
    }
}
