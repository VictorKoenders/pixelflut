#![feature(test)]

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate proptest;
#[cfg(test)]
extern crate test;

#[cfg(test)]
mod integration_test;

mod client;
mod handlers;
mod screen;
mod utils;

pub type Result<T> = std::result::Result<T, ()>;
pub use crate::utils::*;

use clap::{App, Arg, SubCommand};

fn main() {
    let cpu_str = format!(
        "Determine the number of CPU cores to use. 0 for unlimited. Max for this machine is {}.",
        num_cpus::get()
    );
    let matches = App::new("PixelFlut")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .takes_value(true)
                .help("Specify the host to bind to, defaults to 0.0.0.0"),
        ).arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("Specify the port to listen on, defaults to 1234"),
        ).subcommand(SubCommand::with_name("cpu_bound")
            .about("Uses 1 CPU core for rendering and accepting new connections. The incoming clients are split over the remaining cores.")
            .arg(Arg::with_name("num_cpus")
                .short("c")
                .long("num_cpus")
                .takes_value(true)
                .help(&cpu_str)))
        .subcommand(SubCommand::with_name("max_threads")
            .about("Spawns a new thread for each incoming connection."))
        .subcommand(SubCommand::with_name("async")
            .about("Uses async networking to let the OS push messages to the application without waiting for it.")
            .arg(Arg::with_name("num_cpus")
                .short("c")
                .long("num_cpus")
                .takes_value(true)
                .help(&cpu_str))
            )
        .get_matches();

    let host: std::net::IpAddr = matches
        .value_of("host")
        .unwrap_or("0.0.0.0")
        .parse()
        .expect("Host is invalid, IP address expected");
    let port: u16 = matches
        .value_of("port")
        .unwrap_or("1234")
        .parse()
        .expect("Port is invalid");

    utils::initialize_usize();
    utils::initialize_hex();

    let (subcommand_name, subcommand_matches) = matches.subcommand();
    match subcommand_name {
        "cpu_bound" => {
            let mut cpus: usize = subcommand_matches
                .and_then(|m| m.value_of("num_cpus"))
                .unwrap_or("0")
                .parse()
                .expect("num_cpus is invalid");
            if cpus == 0 || cpus > num_cpus::get() {
                cpus = num_cpus::get();
            }
            handlers::cpu_handler::main_loop(host, port, cpus, &handlers::RunIndefinitely);
        }
        "max_threads" => handlers::max_threads::main_loop(host, port, &handlers::RunIndefinitely),
        "async" => {
            let mut cpus: usize = subcommand_matches
                .and_then(|m| m.value_of("num_cpus"))
                .unwrap_or("0")
                .parse()
                .expect("num_cpus is invalid");
            if cpus == 0 || cpus > num_cpus::get() {
                cpus = num_cpus::get();
            }
            handlers::async_handler::main_loop(host, port, cpus, &handlers::RunIndefinitely);
        }
        _ => println!(
            "Missing subcommand, run `{} help` for more information",
            std::env::args().next().unwrap()
        ),
    }
}
