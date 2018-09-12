use client::Client;
use screen::Screen;
use std::io::Read;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::ptr;
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
    pub fn parse_contents(&mut self, bytes_read: usize, mut callback: impl FnMut(&[u8])) {
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
            unsafe {
                ptr::copy(
                    &self.buffer[current_index],
                    &mut self.buffer[0],
                    total_len - current_index,
                );
            }

            self.start_index = total_len - current_index;
        } else {
            self.start_index = total_len - current_index;
        }
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

#[test]
pub fn test_stream_reader() {
    let message = b"PX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\nPX 100 100 FFAABB\n";

    for i in 1..100 {
        let mut reader = StreamReader::default();
        let mut j = 0;
        'j_loop: while j < message.len() {
            for x in 0..i {
                let message_index = j + x;
                let buffer_index = reader.start_index + x;
                if message_index >= message.len() {
                    break 'j_loop;
                }
                reader.buffer[buffer_index] = message[message_index];
            }
            reader.parse_contents(i, |buff| {
                assert_eq!("PX 100 100 FFAABB", ::std::str::from_utf8(buff).unwrap());
            });
            j += i;
        }
    }
}
