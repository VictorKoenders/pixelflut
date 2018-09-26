use client::Client;
use screen::Screen;
use std::io::{ErrorKind, Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread::spawn;
use time;

pub struct Handle {
    counter: Arc<AtomicUsize>,
    sender: Sender<HandlerNotify>,
}

pub enum HandlerNotify {
    AddClient(TcpStream),
}

impl Handle {
    pub fn new() -> Handle {
        let (sender, receiver) = channel();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        spawn(move || {
            run(&receiver, &counter_clone);
        });
        Handle { counter, sender }
    }

    pub fn client_count(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    pub fn add_client(&self, client: TcpStream) {
        self.sender
            .send(HandlerNotify::AddClient(client))
            .expect("Could not send client to a handler");
    }
}

fn run(receiver: &Receiver<HandlerNotify>, counter: &Arc<AtomicUsize>) {
    // The largest message that we support is: (nn = \r\n)
    // 00000000011111111112
    // 12345678901234567890
    // PX XXX YYY RRGGBBnn
    // So we only need to have a buffer of 19 bytes
    let mut buffer = [0u8; 19];

    let mut clients: Vec<(TcpStream, Vec<u8>)> = Vec::new();
    loop {
        match receiver.try_recv() {
            Ok(HandlerNotify::AddClient(client)) => {
                if let Ok(()) = client.set_nonblocking(true) {
                    clients.push((client, Vec::new()));
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            }
            Err(TryRecvError::Disconnected) => {
                let message = b"Server shutting down, bye!\r\n";
                for (mut client, _) in clients {
                    // We don't care if this fails
                    let _ = client.write_all(message);
                }
                return;
            }
            Err(TryRecvError::Empty) => {}
        }

        for index in (0..clients.len()).rev() {
            let len = match clients[index].0.read(&mut buffer) {
                Ok(l) if l > 0 => l,
                Ok(_) => {
                    clients.remove(index);
                    counter.fetch_sub(1, Ordering::Relaxed);
                    continue;
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                Err(_) => {
                    clients.remove(index);
                    counter.fetch_sub(1, Ordering::Relaxed);
                    continue;
                }
            };
            let (stream, client_buffer) = &mut clients[index];
            // We now copy this buffer to our internal buffer, then we split it, then we
            // copy the remainder to our internal buffer again.
            // TODO: Figure out a way to do this more in-place
            client_buffer.extend_from_slice(&buffer[..len]);
            let remaining = split_v1(&client_buffer, |cmd| {
                let _ = Client.handle_message(stream, cmd);
            });
            *client_buffer = remaining;
            client_buffer.truncate(100);
        }
    }
}

fn split_v1(buffer: &[u8], mut cb: impl FnMut(&[u8])) -> Vec<u8> {
    let mut split = buffer.split(|c| c == &b'\n').peekable();
    loop {
        let current = split.next().unwrap();
        if split.peek().is_none() {
            break current.into_iter().cloned().collect();;
        } else {
            cb(current);
        }
    }
}

#[bench]
fn bench_buffer_split_v1(b: &mut ::test::Bencher) {
    let buffer = b"1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n";
    b.iter(|| {
        let result = split_v1(&buffer[..], |c| {
            ::test::black_box(c);
        });
        ::test::black_box(result);
    });
}
#[test]
fn test_buffer_split_v1() {
    let buffer = b"1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n123";
    let result = split_v1(&buffer[..], |c| {
        assert_eq!(b"1234567890", c);
    });
    assert_eq!(&b"123"[..], result.as_slice());
}

#[cfg(test)]
fn split_v2(mut buffer: &[u8], mut cb: impl FnMut(&[u8])) -> Vec<u8> {
    while let Some(index) = buffer.iter().position(|b| *b == b'\n') {
        cb(&buffer[..index]);
        buffer = &buffer[index + 1..];
    }
    buffer.to_vec()
}

#[bench]
fn bench_buffer_split_v2(b: &mut ::test::Bencher) {
    let buffer = b"1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n";
    b.iter(|| {
        let result = split_v2(&buffer[..], |c| {
            ::test::black_box(c);
        });
        ::test::black_box(result);
    });
}
#[test]
fn test_buffer_split_v2() {
    let buffer = b"1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n123";
    let result = split_v2(&buffer[..], |c| {
        assert_eq!(b"1234567890", c);
    });
    assert_eq!(&b"123"[..], result.as_slice());
}

const FRAME_DURATION_NS: u64 = 1_000_000_000 / 60;

pub fn main_loop(host: IpAddr, port: u16, num_cpus: usize) {
    let listener = TcpListener::bind((host, port)).expect("Could not bind on port 1234");
    let mut screen = Screen::init();

    listener
        .set_nonblocking(true)
        .expect("Could not set listener to nonblocking");

    println!("Listening on {}", listener.local_addr().unwrap());
    println!("Handling clients on {} cores", num_cpus - 1);
    println!("Rendering screen on 1 core");

    // We're claiming 1 CPU for the video rendering and accepting new clients
    // The other CPUs will be used to handle clients
    let handler_count = num_cpus - 1;
    let mut handles = Vec::with_capacity(handler_count);
    for _ in 0..handler_count {
        handles.push(Handle::new());
    }

    let mut target_next_frame_time = time::precise_time_ns();
    loop {
        while target_next_frame_time > time::precise_time_ns() {
            match listener.accept() {
                Ok((client, _)) => {
                    let mut lowest = (0, handles[0].client_count());
                    for (index, handle) in handles.iter().enumerate().skip(1) {
                        let count = handle.client_count();
                        if count < lowest.1 {
                            lowest = (index, count);
                        }
                    }
                    handles[lowest.0].add_client(client);
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => panic!("Could not accept new client: {:?}", e),
            }
        }
        target_next_frame_time = time::precise_time_ns() + FRAME_DURATION_NS;
        screen.render();
    }
}
