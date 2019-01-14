use crate::client::Client;
use crate::lines::Lines;
use crate::screen::Screen;
use crate::Result;
use futures::{try_ready, Future, Poll, Stream};
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::timer::Interval;

pub fn main_loop(host: IpAddr, port: u16) {
    loop {
        tokio::run(future::lazy(move || {
            run(host, port);
            run_render_loop();
            Ok(())
        }));
    }
}

fn run_render_loop() {
    let mut screen = Screen::init();
    tokio::spawn(
        Interval::new_interval(Duration::from_millis(33))
            .map_err(|e| {
                eprintln!("Screen failed to update: {:?}", e);
            })
            .for_each(move |_| {
                screen.render();
                Ok(())
            }),
    );
}

fn run(host: IpAddr, port: u16) {
    let listener = TcpListener::bind(&SocketAddr::new(host, port)).expect("Could not bind server");
    tokio::spawn(
        listener
            .incoming()
            .map_err(move |e| {
                eprintln!("{:?}", e);
                println!("You might want to restart this application, as we're not accepting any more clients from now on");
            })
            .for_each(move |socket| {
                tokio::spawn(AsyncClient::new(socket).map_err(|e| {
                }));
                Ok(())
            }),
    );
}

struct AsyncClient {
    addr: SocketAddr,
    lines: Lines,
}

impl AsyncClient {
    pub fn new(stream: TcpStream) -> AsyncClient {
        let addr = stream.peer_addr().unwrap();
        AsyncClient {
            addr,
            lines: Lines::new(stream),
        }
    }
}

impl Future for AsyncClient {
    type Item = ();
    type Error = failure::Error;

    fn poll(&mut self) -> Poll<(), failure::Error> {
        try_ready!(self.lines.poll_flush());
        while let Async::Ready(line) = self.lines.poll()? {
            if let Some(line) = line {
                if let Ok(response) = Client.handle_message_response(&line) {
                    if !response.is_empty() {
                        self.lines.buffer(response);
                        try_ready!(self.lines.poll_flush());
                    }
                }
            } else {
                return Ok(Async::Ready(()));
            }
        }

        Ok(Async::NotReady)
    }
}
