use crate::screen::Screen;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use mio_extras::channel::{channel, Receiver, Sender};
use std::collections::VecDeque;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::{IpAddr, SocketAddr};

pub fn main_loop(
    host: IpAddr,
    port: u16,
    worker_pool_size: usize,
    interrupter: &dyn super::Interrupter,
) {
    let poll = Poll::new().expect("Could not create poll");
    // let mut clients = HashMap::<Token, Client>::new();
    let mut events = Events::with_capacity(1024);
    let listener = TcpListener::bind((host, port).into()).expect("Could not bind listener");
    let screen = Screen::init();

    let mut threads = Vec::with_capacity(worker_pool_size);

    for _ in 0..worker_pool_size {
        threads.push(Worker::spawn(interrupter.clone()));
    }

    std::thread::spawn(move || screen_render_loop(screen));

    poll.registry()
        .register(
            &mut listener,
            Token(1),
            Interest::READABLE | Interest::WRITABLE,
        )
        .expect("Could not register listener");

    let mut next_worker_index = 0;

    while interrupter.is_running() {
        poll.poll(&mut events, Some(std::time::Duration::from_millis(100)))
            .unwrap();
        for _ in events.iter() {
            while let Ok((stream, addr)) = listener.accept() {
                threads[next_worker_index].send(stream, addr);

                next_worker_index = (next_worker_index + 1) % worker_pool_size;
            }
        }
    }
}

struct Worker(Sender<WorkerTask>);

impl Worker {
    pub fn spawn(interrupter: Box<dyn super::Interrupter>) -> Worker {
        let (sender, receiver) = channel();
        std::thread::spawn(move || {
            Worker::run(receiver, interrupter);
        });
        Worker(sender)
    }

    pub fn send(&self, stream: TcpStream, _addr: SocketAddr) {
        self.0
            .send(WorkerTask { stream })
            .expect("Could not send incoming tcp stream to worker");
    }

    fn run(receiver: Receiver<WorkerTask>, interrupter: Box<dyn super::Interrupter>) {
        let poll = Poll::new().expect("Could not create poll");
        let mut buffer = [0u8; 1024];
        let mut events = Events::with_capacity(1024);
        let mut clients = Vec::<Option<Client>>::with_capacity(100_000);
        let mut available_client_indices = VecDeque::<usize>::with_capacity(100_000);

        poll.registry()
            .register(&mut receiver, Token(0), Interest::READABLE)
            .expect("Could not register thread receiver");

        while interrupter.is_running() {
            poll.poll(&mut events, Some(std::time::Duration::from_millis(100)))
                .unwrap();
            for event in events.iter() {
                if event.token() == Token(0) {
                    while let Ok(task) = receiver.try_recv() {
                        let next_token = match available_client_indices.pop_front() {
                            Some(t) => t,
                            None => {
                                clients.push(None);
                                clients.len()
                            }
                        };
                        poll.register(
                            &task.stream,
                            Token(next_token),
                            Ready::readable(),
                            PollOpt::edge(),
                        )
                        .expect("Could not register client stream");
                        clients[next_token - 1] = Some(Client::new(task.stream));
                    }
                } else {
                    let client = &mut clients[event.token().0 - 1].as_mut().unwrap();
                    let mut is_err = client.read(&mut buffer).is_err();
                    if !is_err {
                        is_err = poll
                            .reregister(
                                &client.stream,
                                event.token(),
                                Ready::readable(),
                                PollOpt::edge(),
                            )
                            .is_err();
                    }

                    if is_err {
                        available_client_indices.push_back(event.token().0);
                        clients[event.token().0 - 1] = None;
                    }
                }
            }
        }
    }
}

struct WorkerTask {
    stream: TcpStream,
}

pub fn screen_render_loop(mut screen: Screen) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(33));
        screen.render();
    }
}

pub struct Client {
    pub stream: TcpStream,
    pub buffer: VecDeque<u8>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client {
            stream,
            buffer: VecDeque::with_capacity(1024),
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        let mut length = match self.stream.read(buffer) {
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

        if length + self.buffer.len() > 1024 {
            // make sure we never re-allocate the buffer
            length = 1024 - self.buffer.len();
        }
        for b in &buffer[..length] {
            self.buffer.push_back(*b);
        }

        while let Some(index) = self.buffer.iter().position(|b| b == &b'\n') {
            let end_of_line = if index > 0 && self.buffer[index - 1] == b'\r' {
                index - 1
            } else {
                index
            };

            let line = self.buffer.drain(..end_of_line).collect::<Vec<_>>();
            if let Ok(result) = crate::client::Client.handle_message_response(&line) {
                if !result.is_empty() {
                    self.stream.write_all(&result)?;
                }
            }
            while self.buffer.front() == Some(&b'\r') || self.buffer.front() == Some(&b'\n') {
                self.buffer.pop_front();
            }
        }
        if self.buffer.len() > 100 {
            // we received a lot of data with no newlines, let's just drain the buffer to make sure the client doesn't needlessly allocate memory
            self.buffer.clear();
        }

        Ok(())
    }
}
