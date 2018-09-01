#![feature(const_vec_new)]

extern crate framebuffer;
extern crate num_cpus;
extern crate time;
#[macro_use]
extern crate failure;

mod handlers;
type Result<T> = std::result::Result<T, failure::Error>;

fn main() {
    let num_cpus = num_cpus::get();

    // We're claiming 1 CPU for the video rendering and accepting new clients
    // The other CPUs will be used to handle clients
    let handler_count = num_cpus - 1;
    let mut handles = Vec::with_capacity(handler_count);
    for _ in 0..handler_count {
        handles.push(handlers::client::Handle::new());
    }

    handlers::screen::run(&handles);
}
