#[cfg(unix)]
fn main() {
    use pixelflut::parse::*;

    let arg = ::std::env::args().nth(1);
    match arg.as_deref() {
        Some("color") => {
            let cache = {
                let mut cache = memcache::HexCache::new();
                cache.init();
                cache
            };
            afl::fuzz!(|data: &[u8]| {
                bytewise::parse_color(data);
                std::parse_color(data);
                cache.parse_color(data);
            });
        }
        Some("coordinate") => {
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
        x => {
            panic!(
                "Unknown fuzzing mode: {:?}, expected \"color\" or \"coordinate\"",
                x
            );
        }
    }
}

#[cfg(not(unix))]
fn main() {}
