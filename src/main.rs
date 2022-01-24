use clap::Parser;

fn main() {
    let args = pixelflut::Args::parse();
    pixelflut::parse::initialize();

    let (screen, screen_updater) = pixelflut::screen::new();
    pixelflut::mode::start(args, screen, screen_updater);
}
