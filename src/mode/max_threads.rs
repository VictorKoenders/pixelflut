use crate::{
    client::ClientState,
    screen::{Screen, ScreenUpdater},
};
use std::{
    io::{Read, Write},
    net::TcpListener,
};

pub fn start(args: crate::Args, screen: impl Screen, updater: Option<impl ScreenUpdater>) {
    fn run(args: crate::Args, screen: impl Screen) {
        let listener = TcpListener::bind((args.host, args.port)).expect("Could not bind listener");
        while let Ok((stream, _)) = listener.accept() {
            let screen = screen.clone();
            std::thread::spawn(move || run_client(stream, screen));
        }
    }

    if let Some(mut updater) = updater {
        std::thread::spawn(|| run(args, screen));
        while updater.running() {
            updater.update();
        }
    } else {
        run(args, screen);
    }
}

fn run_client(mut stream: std::net::TcpStream, screen: impl Screen) -> std::io::Result<()> {
    let mut client = ClientState::default();
    while screen.running() {
        let len = stream.read(client.recv_buffer())?;
        if len == 0 {
            return Ok(());
        }
        if let Some(response) = client.parse_buffer(&screen, len) {
            stream.write_all(response.as_bytes().as_ref())?;
        }
    }
    Ok(())
}
