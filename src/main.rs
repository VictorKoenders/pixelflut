mod client;
mod mode;
mod screen;

use clap::Parser;

fn main() {
    let args = crate::Args::parse();
    let (screen, screen_updater) = crate::screen::new();
    mode::start(args, screen, screen_updater);
}

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
    /// What address to listen on. Defaults to "localhost"
    #[clap(short, long, default_value_t = String::from("localhost"))]
    pub host: String,

    /// What port to listen on. Defaults to 1234
    #[clap(short, long, default_value_t = 1234)]
    pub port: u16,

    /// How many cores to use with multithreading. Does not affect max-threads
    #[clap(short, long)]
    pub core_count: Option<usize>,
}
