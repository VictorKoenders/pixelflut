use clap::Parser;

fn main() {
    let args = pixelflut::Args::parse();
    pixelflut::parse::initialize();

    if let Some(path) = &args.export_frames {
        if !path.exists() {
            std::fs::create_dir_all(path).expect("Could not create folder");
        }
    }

    let (screen, screen_updater) = pixelflut::screen::new(&args);
    pixelflut::mode::start(args, screen, screen_updater);
}
