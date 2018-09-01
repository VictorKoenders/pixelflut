use handlers::screen;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::spawn;
use Result;

const HELP_MESSAGE: &str = r#"Possible commands:
- PX <x> <y> <rgbhex>: Set the pixel at X/Y the given RGB value. This needs to be a 6-character HEX value, e.g. 000000 for black and FFFFFF for white
- SIZE: returns the size of the screen, in pixels
- HELP: returns this help
"#;

pub struct Handle {
    counter: Arc<Mutex<usize>>,
    sender: Sender<HandlerNotify>,
}

pub enum HandlerNotify {
    AddClient(TcpStream),
    Stop,
}

impl Handle {
    pub fn new() -> Handle {
        let (sender, receiver) = channel();
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();
        spawn(move || {
            run(receiver, counter_clone);
        });
        Handle { counter, sender }
    }

    pub fn client_count(&self) -> usize {
        *self.counter.lock().unwrap()
    }

    pub fn add_client(&self, client: TcpStream) {
        self.sender
            .send(HandlerNotify::AddClient(client))
            .expect("Could not send client to a handler");
    }
}

fn run(receiver: Receiver<HandlerNotify>, counter: Arc<Mutex<usize>>) {
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
                    *counter.lock().expect("Could not access Mutex") += 1;
                }
            }
            Err(TryRecvError::Disconnected) | Ok(HandlerNotify::Stop) => {
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
                    *counter.lock().expect("Could not access Mutex") -= 1;
                    continue;
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    continue;
                }
                Err(_) => {
                    clients.remove(index);
                    *counter.lock().expect("Could not access Mutex") -= 1;
                    continue;
                }
            };
            clients[index].1.extend_from_slice(&buffer[..len]);
            if let Some(i) = clients[index].1.iter().position(|c| c == &b'\n') {
                if let Err(_) = handle_client_update(&mut clients, index, i) {}
            }
        }
    }
}

fn handle_client_update(
    clients: &mut Vec<(TcpStream, Vec<u8>)>,
    index: usize,
    i: usize,
) -> Result<()> {
    let line = clients[index].1.drain(..i + 1).collect::<Vec<_>>();
    let str = ::std::str::from_utf8(&line)?;
    println!("{:?}", str);
    let mut iter = str.trim().split(' ');

    match iter.next() {
        Some("PX") => {
            // Set pixel
            macro_rules! unwrap_or_return {
                ($stmt:expr) => {
                    match $stmt {
                        Some(n) => n,
                        None => return Ok(()),
                    }
                };
            }
            let x: usize = unwrap_or_return!(iter.next()).parse()?;
            let y: usize = unwrap_or_return!(iter.next()).parse()?;
            let format: &str = unwrap_or_return!(iter.next());
            if format.len() != 6 {
                bail!("Format is invalid length");
            }

            let r = u8::from_str_radix(&format[0..2], 16)?;
            let g = u8::from_str_radix(&format[2..4], 16)?;
            let b = u8::from_str_radix(&format[4..], 16)?;
            screen::set_pixel((x, y), (r, g, b));
        }
        Some("HELP") => {
            // Send help :'(
            clients[index].0.write_all(HELP_MESSAGE.as_bytes())?;
        }
        Some("SIZE") => {
            // Send screen size
            clients[index].0.write_all(screen::get_screen_size())?;
        }
        _ => { /* ignored FeelsBadMan */ }
    }
    Ok(())
}
