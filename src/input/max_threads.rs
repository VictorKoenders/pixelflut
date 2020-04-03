use crate::config::Config;
use crate::output::ScreenBuffer;
use crate::parse::Parse;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct MaxThreads;

impl super::Input for MaxThreads {
    fn spawn<P: Parse + 'static, S: ScreenBuffer + 'static>(
        config: Config,
        parser: P,
        buffer: S,
    ) -> Self {
        thread::spawn(move || listen_loop(config, parser, buffer));
        MaxThreads
    }
}

fn listen_loop<P: Parse + 'static, S: ScreenBuffer + 'static>(
    config: Config,
    parser: P,
    buffer: S,
) {
    let listener = TcpListener::bind((config.host, config.port)).expect("Could not bind on host");
    loop {
        let (socket, _) = listener.accept().expect("Could not accept new connection");
        let parser = parser.clone();
        let buffer = buffer.clone();
        thread::spawn(move || handle_client(socket, parser, buffer));
    }
}

fn handle_client<P: Parse + 'static, S: ScreenBuffer + 'static>(
    mut socket: TcpStream,
    mut parser: P,
    buffer: S,
) {
    loop {
        let n = match socket.read(parser.write_buffer()) {
            Ok(0) => break,
            Err(_) => break,
            Ok(n) => n,
        };
        if let Some(response) = parser.parse(n, &buffer) {
            if socket.write_all(response.as_bytes()).is_err() {
                break;
            }
        }
    }
}
