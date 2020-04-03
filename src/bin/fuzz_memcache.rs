#[macro_use]
extern crate afl;
extern crate pixelflut;

use pixelflut::config::Config;
use pixelflut::output::{Fake, Screen};
use pixelflut::parse::{MemCache, Parse};

fn main() {
    let mut config = Config::default();
    config.screen_dimensions = (10, 10);
    let screen = Fake::create(config.clone());
    let mut memcache = MemCache::new(config);

    fuzz!(|data: &[u8]| {
        let data = if data.len() > 1024 {
            &data[..1024]
        } else {
            data
        };
        memcache.write_buffer()[..data.len()].copy_from_slice(data);
        memcache.parse(data.len(), &screen);
    });
}
