use crate::screen::Screen;
use hashbrown::HashMap;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Poll, PollOpt, Ready, Token};
use std::io::{Error, ErrorKind, Read, Write};
use std::net::IpAddr;

pub fn main_loop(host: IpAddr, port: u16) {
    let poll = Poll::new().expect("Could not create poll");
    let mut clients = HashMap::<Token, Client>::new();
    let mut buffer = [0u8; 1024];
    let mut events = Events::with_capacity(1024);
    let listener = TcpListener::bind(&(host, port).into()).expect("Could not bind listener");
    let mut token_generator = TokenGenerator::new();
    let server_token = token_generator.next();
    let screen = Screen::init();

    std::thread::spawn(move || screen_render_loop(screen));

    poll.register(&listener, server_token, Ready::readable(), PollOpt::edge())
        .expect("Could not register listener");

    loop {
        poll.poll(&mut events, None).unwrap();
        for event in events.iter() {
            if event.token() == server_token {
                while let Ok((stream, _addr)) = listener.accept() {
                    let token = token_generator.next();
                    poll.register(&stream, token, Ready::readable(), PollOpt::edge())
                        .expect("Could not register client");
                    clients.insert(token, Client::new(stream));
                }
            } else {
                let mut should_remove = false;
                if let Some(client) = clients.get_mut(&event.token()) {
                    if client.read(&mut buffer).is_err() {
                        should_remove = true;
                    }
                }

                if should_remove {
                    clients.remove(&event.token());
                    token_generator.release(event.token());
                }
            }
        }
    }
}

pub fn screen_render_loop(mut screen: Screen) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(33));
        screen.render();
    }
}

pub struct Client {
    pub stream: TcpStream,
    pub buffer: Vec<u8>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client {
            stream,
            buffer: Vec::with_capacity(1024),
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        loop {
            let length = match self.stream.read(buffer) {
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    return Ok(());
                }
                Err(e) => return Err(e),
                Ok(l) => l,
            };

            if length == 0 {
                // pipe broken
                return Err(Error::new(ErrorKind::Other, "Broken pipe"));
            }

            self.buffer.extend_from_slice(&buffer[..length]);

            while let Some(index) = self.buffer.iter().position(|b| b == &b'\n') {
                let end_of_line = if index > 0 && self.buffer[index - 1] == b'\r' {
                    index - 1
                } else {
                    index
                };

                let result = match crate::client::Client
                    .handle_message_response(&self.buffer[..end_of_line])
                {
                    Ok(r) => r,
                    Err(()) => return Err(Error::new(ErrorKind::Other, "Could not handle message")),
                };
                if !result.is_empty() {
                    self.stream.write_all(result)?;
                }

                self.buffer.drain(..index);
            }
        }
    }
}

pub struct TokenGenerator {
    released: Vec<Token>,
    next: usize,
}

impl TokenGenerator {
    pub fn new() -> TokenGenerator {
        TokenGenerator {
            released: Vec::new(),
            next: 1,
        }
    }

    pub fn next(&mut self) -> Token {
        if let Some(t) = self.released.pop() {
            t
        } else {
            let result = Token(self.next);
            self.next += 1;
            result
        }
    }

    pub fn release(&mut self, token: Token) {
        self.released.push(token);
    }
}
