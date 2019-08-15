use crate::client::Client;
use crate::screen::Screen;
use std::io::{ErrorKind, Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread::spawn;
use time;

#[cfg(test)]
use std::collections::VecDeque;

pub struct Handle {
    counter: Arc<AtomicUsize>,
    sender: Sender<HandlerNotify>,
    handle: std::thread::JoinHandle<()>,
}

pub enum HandlerNotify {
    AddClient(TcpStream),
}

impl Handle {
    pub fn new(interrupter: Box<dyn super::Interrupter>) -> Handle {
        let (sender, receiver) = channel();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        let handle = spawn(move || {
            run(&receiver, &counter_clone, interrupter);
        });
        Handle {
            counter,
            sender,
            handle,
        }
    }

    pub fn join(self) {
        self.handle.join().expect("Could not wait for handle");
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

fn run(
    receiver: &Receiver<HandlerNotify>,
    counter: &Arc<AtomicUsize>,
    interrupter: Box<dyn super::Interrupter>,
) {
    // The largest message that we support is: (nn = \r\n)
    // 00000000011111111112
    // 12345678901234567890
    // PX XXX YYY RRGGBBnn
    // So we only need to have a buffer of 19 bytes
    let mut buffer = [0u8; 19];

    let mut clients: Vec<(TcpStream, Vec<u8>)> = Vec::with_capacity(100_000);
    while interrupter.is_running() {
        match receiver.try_recv() {
            Ok(HandlerNotify::AddClient(client)) => {
                if let Ok(()) = client.set_nonblocking(true) {
                    clients.push((client, Vec::with_capacity(100_000)));
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
            let remaining = split(&client_buffer, |cmd| {
                let _ = Client.handle_message(stream, cmd);
            });
            *client_buffer = remaining;
            client_buffer.truncate(100);
        }
    }
}

const FRAME_DURATION_NS: u64 = 1_000_000_000 / 30;

pub fn main_loop(host: IpAddr, port: u16, num_cpus: usize, interrupter: &dyn super::Interrupter) {
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
        handles.push(Handle::new(interrupter.clone()));
    }

    let mut target_next_frame_time = time::precise_time_ns();
    while interrupter.is_running() {
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

    for handle in handles {
        handle.join();
    }
}

#[cfg(test)]
fn split_v1(buffer: &[u8], mut cb: impl FnMut(&[u8])) -> Vec<u8> {
    let mut split = buffer.split(|c| c == &b'\n').peekable();
    loop {
        let current = split.next().unwrap();
        if split.peek().is_none() {
            break current.to_vec();
        } else {
            cb(current);
        }
    }
}

fn split(mut buffer: &[u8], mut cb: impl FnMut(&[u8])) -> Vec<u8> {
    while let Some(index) = buffer.iter().position(|b| *b == b'\n') {
        cb(&buffer[..index]);
        buffer = &buffer[index + 1..];
    }
    buffer.to_vec()
}

#[cfg(test)]
fn split_v3(buffer: &mut Vec<u8>, mut cb: impl FnMut(&[u8])) {
    let mut offset = 0;
    let mut i = 0;
    while i < buffer.len() {
        if unsafe { buffer.get_unchecked(i) } == &b'\n' {
            cb(&buffer[offset..i]);
            offset = i + 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    let new_length = buffer.len() - offset;
    unsafe {
        std::ptr::copy_nonoverlapping(
            buffer.get_unchecked(offset),
            buffer.get_unchecked_mut(0),
            new_length,
        );
        buffer.set_len(new_length);
    }
}

#[cfg(test)]
fn split_v4(buffer: &mut VecDeque<u8>, mut cb: impl FnMut(&[u8])) {
    while let Some(i) = buffer.iter().position(|b| b == &b'\n') {
        let line = buffer.drain(..=i).take(i - 1).collect::<Vec<_>>();
        let mut line_slice = &line[..];
        while line_slice.last() == Some(&b'\n') || line_slice.last() == Some(&b'\r') {
            line_slice = &line_slice[..line_slice.len() - 1];
        }
        cb(line_slice);
    }
}

macro_rules! test_and_bench {
    ($fn_name:ident) => {
        test_and_bench!($fn_name, $fn_name);
    };
    ($mod_name:ident, $fn_name:ident) => {
        pub mod $mod_name {

            #[bench]
            fn bench(b: &mut ::test::Bencher) {
                let vec: Vec<u8> = Vec::from(&b"1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n"[..]);
                b.iter(|| {
                    let mut cloned: Vec<u8> = vec.clone();
                    let result: Vec<u8> = super::$fn_name(&cloned[..], |c| {
                        ::test::black_box(c);
                    });
                    cloned = result;
                    ::test::black_box(cloned);
                });
            }
            #[test]
            fn test() {
                let buffer = Vec::from(&b"1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n123"[..]);
                let result = super::$fn_name(&buffer[..], |c| {
                    assert_eq!(b"1234567890", c);
                });
                assert_eq!(&b"123"[..], result.as_slice());
            }
        }
    };
}

test_and_bench!(split_v1);
test_and_bench!(split_v2, split);

#[bench]
fn bench_buffer_split_v3(b: &mut ::test::Bencher) {
    let vec: Vec<u8> = Vec::from(&b"1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n"[..]);
    b.iter(|| {
        let mut cloned = vec.clone();
        split_v3(&mut cloned, |c| {
            ::test::black_box(c);
        });
        ::test::black_box(cloned);
    });
}

#[test]
fn test_buffer_split_v3() {
    let mut buffer = Vec::from(&b"1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n123"[..]);
    split_v3(&mut buffer, |c| {
        assert_eq!(b"1234567890", c);
    });
    assert_eq!(&b"123"[..], buffer.as_slice());
}

#[bench]
fn bench_buffer_split_v4(b: &mut ::test::Bencher) {
    let buffer: VecDeque<u8> = Vec::from(&b"1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n123"[..]).into();
    b.iter(|| {
        let mut cloned: VecDeque<u8> = buffer.clone();
        split_v4(&mut cloned, |c| {
            ::test::black_box(c);
        });
        ::test::black_box(cloned);
    });
}
#[bench]
fn bench_buffer_split_v4_push_front(b: &mut ::test::Bencher) {
    let source = Vec::from(&b"1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n123"[..]);
    let mut buffer: VecDeque<u8> = VecDeque::with_capacity(1024);
    for i in source {
        buffer.push_front(i);
    }
    b.iter(|| {
        let mut cloned: VecDeque<u8> = buffer.clone();
        split_v4(&mut cloned, |c| {
            ::test::black_box(c);
        });
        ::test::black_box(cloned);
    });
}
#[test]
fn test_buffer_split_v4() {
    let source = Vec::from(&b"1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n123"[..]);
    let mut buffer: VecDeque<u8> = VecDeque::with_capacity(1024);
    for i in source {
        buffer.push_front(i);
    }
    split_v4(&mut buffer, |c| {
        assert_eq!(b"1234567890", c);
    });
    let remaining = buffer.into_iter().collect::<Vec<_>>();
    assert_eq!(&b"123"[..], remaining.as_slice());
}
