use clap::Parser;
use std::path::PathBuf;

pub mod client;
pub mod mode;
pub mod parse;
pub mod screen;

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

    /// Set a folder to where the frames should be exported each frame.
    #[clap(short, long)]
    pub export_frames: Option<PathBuf>,
}
