use pixelflut::config::Config;
use pixelflut::input::{Input, MaxThreads, Tokio};
use pixelflut::output::{MiniFb, Screen};
use pixelflut::parse::{MemCache, Parse};

fn main() {
    let config = Config::default();
    let mut output = MiniFb::create(config.clone());
    let parser = MemCache::new(config.clone());
    let _tokio = Tokio::spawn(config, parser, output.get_buffer());
    // let _max_threads = MaxThreads::spawn(config, parser, output.get_buffer());

    while output.is_running() {
        output.draw();
    }
    std::process::exit(0);
}
