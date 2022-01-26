use pixelflut::parse::*;

fn main() {
    let mut memcache = memcache::NumCache::new();
    memcache.init();

    afl::fuzz!(|data: &[u8]| {
        let result = bytewise::parse_coordinate(data);
        assert_eq!(result, std::parse_coordinate(data));

        match (result, memcache.parse_coordinate(data)) {
            (Some(x), None) if x.0 > MAX_VALID_NUMBER => {}
            (Some(x), Some(y)) if x == y => {}
            (None, None) => {}
            _ => panic!("bytewise and memcache did not agree on {:?}", data),
        }
    });
}
