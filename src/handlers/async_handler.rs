use crate::client::Client;
use crate::lines::Lines;
use crate::screen::Screen;
use futures::{Future, Poll, Stream};
use std::net::{IpAddr, SocketAddr};
use std::thread;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

pub fn main_loop(host: IpAddr, port: u16) {
    let screen = Screen::init();
    thread::spawn(move || run_render_loop(screen));
    tokio::run(future::lazy(move || {
        run(host, port);
        Ok(())
    }));
}

fn run_render_loop(mut screen: Screen) {
    loop {
        thread::sleep(Duration::from_millis(33));
        screen.render();
    }
}

fn run(host: IpAddr, port: u16) {
    let listener = TcpListener::bind(&SocketAddr::new(host, port)).expect("Could not bind server");
    println!("Listening on {:?}", SocketAddr::new(host, port));
    tokio::spawn(
        listener
            .incoming()
            .map_err(move |e| {
                eprintln!("{:?}", e);
                println!("You might want to restart this application, as we're not accepting any more clients from now on");
            })
            .for_each(move |socket| {
                tokio::spawn(AsyncClient::new(socket).map_err(|_| ()));
                Ok(())
            })
            .and_then(|_| {
                eprintln!("Listener loop ended, please restart");
                Ok(())
            }),
    );
}

struct AsyncClient {
    lines: Lines,
}

impl AsyncClient {
    pub fn new(stream: TcpStream) -> AsyncClient {
        AsyncClient {
            lines: Lines::new(stream),
        }
    }
}

impl Future for AsyncClient {
    type Item = ();
    type Error = failure::Error;

    fn poll(&mut self) -> Poll<(), failure::Error> {
        self.lines.poll_flush()?;
        while let Async::Ready(line) = self.lines.poll()? {
            if let Some(line) = line {
                let line = if line.last() == Some(&b'\r') {
                    &line[..line.len() - 1]
                } else {
                    &line[..]
                };
                if let Ok(response) = Client.handle_message_response(&line) {
                    if !response.is_empty() {
                        self.lines.buffer(response);
                        self.lines.poll_flush()?;
                    }
                }
            } else {
                return Ok(Async::Ready(()));
            }
        }

        Ok(Async::NotReady)
    }
}
