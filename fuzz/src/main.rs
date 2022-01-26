use pixelflut::parse::*;

fn main() {
    let cache = {
        let mut cache = memcache::NumCache::new();
        cache.init();
        cache
    };
    afl::fuzz!(|data: &[u8]| {
        bytewise::parse_coordinate(data);
        std::parse_coordinate(data);
        cache.parse_coordinate(data);
    });
}
