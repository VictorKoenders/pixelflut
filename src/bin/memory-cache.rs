fn main() {
    use pixelflut::parse::*;
    use rand::{thread_rng, Rng};

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
    let mut cache = memcache::NumCache::new();
    cache.init();

    let start = ::std::time::Instant::now();
    println!("Running for 60 seconds");

    while start.elapsed().as_secs() < 60 {
        let mut buffer = input.as_bytes();

        let mut idx = 0;

        while let Some((num, remaining)) = cache.parse_coordinate(buffer) {
            assert_eq!(num, expected[idx]);
            idx += 1;
            buffer = remaining;
        }
        assert_eq!(idx, expected.len());
        assert!(buffer.is_empty(), "Remaining bytes: {:?}", buffer);
    }
}
