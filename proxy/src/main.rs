#![feature(const_vec_new)]

extern crate clap;
extern crate snap;

use clap::{App, Arg};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread::{sleep, spawn};
use std::time::Duration;

static mut WIDTH: usize = 0;
static mut HEIGHT: usize = 0;
static mut BUFFER: Vec<Vec<(u8, u8, u8)>> = Vec::new();

const SEPERATOR: &u8 = &b' ';
const CARRIAGE_RETURN: &u8 = &b'\r';
const NEWLINE: &u8 = &b'\n';
const CLIENT_DISCONNECT_MSG: &[u8] = b"Failed to read; disconnecting";
const CLIENT_ERROR_SIZE: &[u8] = b"Invalid coordinate. Request SIZE to get the allowed dimensions";
const CLIENT_ERROR_COLOR: &[u8] =
    b"Invalid color, must be exactly 6 bytes and include only 0-9 or a-f";
const CLIENT_HELP_MSG: &[u8] = b"Welcome!\r\n\r\nYou have succesfully connected to the pixelflut server.\r\n\r\nCommands are newline seperated (\\n or \\r\\n). You can send the following commands:\r\nHELP                   Sends this help text\r\nSIZE                   Returns the size of the screen in the format 'SIZE <xx> <yy>\n', with x and y being exclusive (0 <= n < x/y)\r\nPX <xx> <yy> <rrggbb>  Sets a specific pixel on this screen. the <rrggbb> format is hexadecimal.\r\n\r\nExample:\r\nPX 15 30 ff0000        Sets pixel at (15, 30) to red (0xFF red, 0x00 green, 0x00 blue)\r\n";

fn send_buffer_to_remote(addr: SocketAddr) {
    let stream = match TcpStream::connect(addr) {
        Ok(s) => s,
        Err(e) => {
            println!("Could not connect to remote host");
            println!("{:?}", e);
            std::process::exit(-1);
        }
    };
    let mut buff = [0u8; 5];
    let mut buffer = Vec::new();
    buffer.resize(unsafe { WIDTH * HEIGHT * 3 }, 0);
    let mut snap = snap::Writer::new(stream);
    loop {
        unsafe {
            let mut index = 0;
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    buffer[index] = BUFFER[x][y].0;
                    buffer[index + 1] = BUFFER[x][y].1;
                    buffer[index + 2] = BUFFER[x][y].2;
                    index += 3;
                }
            }
        };
        {
            snap.write(&buffer).unwrap();
            snap.flush().unwrap();
        }
        let len = snap
            .get_ref()
            .read(&mut buff)
            .expect("Could not read confirmation from remote");
        if len != 4 || &buff[..len] != b"ACK\n" {
            panic!(
                "Failed to receive an ACK from the server, got {:?} (len {})",
                &buff[..len],
                len
            );
        }
        sleep(Duration::from_millis(100));
    }
}

fn get_number_from_slice(slice: &[u8], max: usize) -> Option<usize> {
    let mut result = 0;
    for n in slice {
        if n < &b'0' || n > &b'9' {
            return None;
        }
        result = result * 10 + (n - b'0') as usize;
        if result > max {
            return None;
        }
    }
    Some(result)
}

fn get_hex(s: &[u8]) -> Option<(u8, u8, u8)> {
    if s.len() != 6 {
        return None;
    }
    fn get_hex_value(v: u8) -> Option<u8> {
        if v >= b'0' && v <= b'9' {
            Some(v - b'0')
        } else if v >= b'A' && v <= b'F' {
            Some(v - b'A' + 10)
        } else if v >= b'a' && v <= b'f' {
            Some(v - b'a' + 10)
        } else {
            None
        }
    }

    Some((
        get_hex_value(s[0])? * 16 + get_hex_value(s[1])?,
        get_hex_value(s[2])? * 16 + get_hex_value(s[3])?,
        get_hex_value(s[4])? * 16 + get_hex_value(s[5])?,
    ))
}

fn handle_client(mut client: TcpStream) {
    // largest data we receive is:
    // 000000000011111111112
    // 012345678901234567890
    // PX XXX YYY RRGGBB\r\n
    let mut buff = [0u8; 21];
    loop {
        let len = match client.read(&mut buff) {
            Ok(l) => l,
            Err(_) => {
                let _ = client.write(CLIENT_DISCONNECT_MSG);
                return;
            }
        };
        let mut split =
            buff[..len].split(|b| b == SEPERATOR || b == CARRIAGE_RETURN || b == NEWLINE);
        match split.next() {
            Some(b"PX") => {
                let x = match split
                    .next()
                    .and_then(|s| get_number_from_slice(s, unsafe { WIDTH }))
                {
                    Some(n) => n,
                    _ => {
                        if let Err(_) = client.write(CLIENT_ERROR_SIZE) {
                            return;
                        }
                        continue;
                    }
                };
                let y = match split
                    .next()
                    .and_then(|s| get_number_from_slice(s, unsafe { HEIGHT }))
                {
                    Some(n) => n,
                    _ => {
                        if let Err(_) = client.write(CLIENT_ERROR_SIZE) {
                            return;
                        }
                        continue;
                    }
                };
                let color = match split.next().and_then(get_hex) {
                    Some(n) => n,
                    _ => {
                        if let Err(_) = client.write(CLIENT_ERROR_COLOR) {
                            return;
                        }
                        continue;
                    }
                };

                unsafe {
                    BUFFER[x][y] = color;
                }
            }
            Some(b"SIZE") => {
                if let Err(_) = client
                    .write(format!("SIZE {} {}\n", unsafe { WIDTH }, unsafe { HEIGHT }).as_bytes())
                {
                    return;
                }
            }
            Some(b"HELP") => {
                if let Err(_) = client.write(CLIENT_HELP_MSG) {
                    return;
                }
            }
            _ => {} // ignored
        }
    }
}

fn main() {
    let matches = App::new("Pixelflut Proxy")
        .arg(
            Arg::with_name("remote")
                .long("remote")
                .short("r")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .takes_value(true)
                .help("Listens on a random port if omitted"),
        )
        .arg(
            Arg::with_name("width")
                .long("width")
                .short("w")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("height")
                .long("height")
                .short("h")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    unsafe {
        WIDTH = matches
            .value_of("width")
            .expect("Missing value 'width'")
            .parse()
            .expect("Width is not a valid number");
        HEIGHT = matches
            .value_of("height")
            .expect("Missing value 'height'")
            .parse()
            .expect("Height is not a valid number");

        BUFFER.resize(WIDTH, Vec::new());
        for i in 0..WIDTH {
            BUFFER[i].resize(HEIGHT, (0, 0, 0xff));
        }
    }

    let addr = format!("0.0.0.0:{}", matches.value_of("port").unwrap_or("0"));

    let listener = TcpListener::bind(addr).expect("Could not bind to port");
    println!(
        "Listening on {}",
        listener.local_addr().expect("Could not read local addr")
    );
    let remote_addr = matches
        .value_of("remote")
        .expect("Missing required field 'remote'");
    println!("Connecting to {:?}", remote_addr);
    let remote_addr: SocketAddr = remote_addr
        .parse()
        .expect("Remote is not a valid IP address");

    spawn(move || send_buffer_to_remote(remote_addr));

    loop {
        let (client, _) = listener.accept().expect("Could not accept client");
        spawn(move || handle_client(client));
    }
}
