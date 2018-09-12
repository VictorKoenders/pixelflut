use client::Client;
use screen::Screen;
use std::io::Read;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::thread::{sleep, spawn};
use std::time::Duration;
use Result;

pub fn main_loop(host: IpAddr, port: u16) {
    let screen = Screen::init();

    spawn(move || render_loop(&screen));

    let listener = TcpListener::bind((host, port))
        .unwrap_or_else(|e| panic!("Could not listen on {}:{}: {:?}", host, port, e));
    println!("Listening on {}", listener.local_addr().unwrap());

    loop {
        let (socket, _) = listener
            .accept()
            .expect("Could not accept new TCP connection");
        spawn(|| {
            let _ = run_client(socket);
        });
    }
}

fn render_loop(screen: &Screen) {
    loop {
        screen.render();
        sleep(Duration::from_millis(33));
    }
}

struct StreamReader {
    pub buffer: [u8; 100],
    pub start_index: usize,
}

impl Default for StreamReader {
    fn default() -> StreamReader {
        StreamReader {
            buffer: [0u8; 100],
            start_index: 0,
        }
    }
}

impl StreamReader {
    #[cfg(test)]
    pub fn parse_contents_v1(&mut self, bytes_read: usize, mut callback: impl FnMut(&[u8])) {
        let total_len = self.start_index + bytes_read;
        let mut current_index = 0;

        while let Some(index) = self.buffer[..total_len]
            .iter()
            .skip(current_index)
            .position(|b| b == &b'\n')
        {
            callback(&self.buffer[current_index..current_index + index]);
            current_index += index + 1;
        }

        if current_index > 0 {
            let remaining: Vec<u8> = self.buffer[current_index..total_len].to_vec();
            self.buffer[..remaining.len()].clone_from_slice(&remaining[..]);

            self.start_index = total_len - current_index;
            debug_assert_eq!(self.start_index, remaining.len());
        } else {
            self.start_index = total_len - current_index;
        }
    }
    pub fn parse_contents(&mut self, bytes_read: usize, mut callback: impl FnMut(&[u8])) {
        let remaining = {
            let bytes = &self.buffer[..self.start_index + bytes_read];
            let mut split = bytes.split(|b| b == &b'\n').peekable();
            loop {
                if let Some(next) = split.next() {
                    let after_this = split.peek();
                    if after_this.is_none() {
                        break next.to_vec();
                    } else {
                        callback(next);
                    }
                } else {
                    unreachable!();
                }
            }
        };

        self.buffer[..remaining.len()].copy_from_slice(remaining.as_slice());
        self.start_index = remaining.len();
    }
}

fn run_client(mut socket: TcpStream) -> Result<()> {
    let mut reader = StreamReader::default();
    loop {
        let len = socket.read(&mut reader.buffer[reader.start_index..])?;
        if len == 0 {
            break;
        }
        reader.parse_contents(len, |buff| {
            let _ = Client.handle_message(&mut socket, buff);
        });
    }
    Ok(())
}

#[cfg(test)]
const LONG_MESSAGE: &[u8] = b"PX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\n";

#[bench]
pub fn bench_stream_reader_v1(b: &mut ::test::Bencher) {
    let mut reader = StreamReader::default();
    b.iter(|| {
        for chunk in LONG_MESSAGE.chunks(100) {
            reader.start_index = 0;
            reader.buffer[..chunk.len()].copy_from_slice(chunk);
            reader.parse_contents_v1(100, |buff| {
                ::test::black_box(buff);
            });
        }
    });
}

#[bench]
pub fn bench_stream_reader_v2(b: &mut ::test::Bencher) {
    let mut reader = StreamReader::default();
    b.iter(|| {
        for chunk in LONG_MESSAGE.chunks(100) {
            reader.start_index = 0;
            reader.buffer[..chunk.len()].copy_from_slice(chunk);
            reader.parse_contents(100, |buff| {
                ::test::black_box(buff);
            });
        }
    });
}

#[test]
pub fn test_stream_reader() {
    use std::str;

    for i in 1..100 {
        println!("i = {}", i);
        let mut reader = StreamReader::default();
        let mut j = 0;
        'j_loop: while j < LONG_MESSAGE.len() {
            let mut bytes_read = 0;
            let mut message_index = j;
            for x in 0..i {
                let buffer_index = reader.start_index + x;
                if message_index >= LONG_MESSAGE.len() {
                    break 'j_loop;
                }
                if buffer_index >= reader.buffer.len() {
                    break;
                }
                reader.buffer[buffer_index] = LONG_MESSAGE[message_index];
                bytes_read += 1;
                message_index += 1;
            }
            if bytes_read == 0 {
                break;
            }
            println!(
                "Buffer is {:?}",
                str::from_utf8(&reader.buffer[..reader.start_index + bytes_read])
            );
            let mut count = 0;
            let expected_count = (reader.start_index + bytes_read) / "PX 100 100 FFAABB\n".len();
            reader.parse_contents(bytes_read, |buff| {
                assert_eq!("PX 100 100 FFAABB", ::std::str::from_utf8(buff).unwrap());
                count += 1;
            });
            println!("Expected {} messages, got {}", expected_count, count);
            assert_eq!(expected_count, count);
            j += bytes_read;
        }
    }
}

#[cfg(test)]
proptest! {
    #[test]
    fn fuzz_stream_reader(parts: [[u8; 25];4]) {
        let flat: Vec<u8> = parts.iter().flatten().cloned().collect();
        let mut split = flat.split(|b| b == &b'\n');

        let mut reader = StreamReader::default();
        reader.buffer[..].copy_from_slice(&flat);

        reader.parse_contents(100, |buff| {
            assert_eq!(split.next().unwrap(), buff);
        });
        let last = split.next().unwrap();
        assert_eq!(None, split.next());

        assert_eq!(reader.start_index, last.len());
        assert_eq!(last, &reader.buffer[..reader.start_index]);
    }
}
