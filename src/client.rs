use crate::screen::Screen;
use std::io::Write;
use std::net::TcpStream;
#[cfg(test)]
use test::{black_box, Bencher};

const HELP_MESSAGE: &[u8] = br#"Possible commands:
- PX <x> <y> <rgbhex>: Set the pixel at X/Y the given RGB value. This needs to be a 6-character HEX value, e.g. 000000 for black and FFFFFF for white
- SIZE: returns the size of the screen, in pixels
- HELP: returns this help
"#;

pub struct Client;

impl Client {
    pub fn handle_message_response(&self, buffer: &[u8]) -> Result<&'static [u8], ()> {
        handle_message_v2(buffer)
    }

    pub fn handle_message(&self, stream: &mut TcpStream, buffer: &[u8]) -> Result<(), ()> {
        let slice = handle_message_v2(buffer)?;
        if !slice.is_empty() {
            stream.write_all(slice).map_err(|_| ())?;
        }
        Ok(())
    }
}

#[cfg(test)]
fn handle_message_v1(buffer: &[u8]) -> Result<&'static [u8], ()> {
    let str = ::std::str::from_utf8(&buffer).map_err(|_| ())?;
    let mut iter = str.trim().split(' ');

    match iter.next() {
        Some("PX") => {
            // Set pixel
            macro_rules! unwrap_or_return {
                ($stmt:expr) => {
                    match $stmt {
                        Some(n) => n,
                        None => return Err(()),
                    }
                };
            }
            let x: usize = unwrap_or_return!(iter.next()).parse().map_err(|_| ())?;
            let y: usize = unwrap_or_return!(iter.next()).parse().map_err(|_| ())?;
            let format: &str = unwrap_or_return!(iter.next());
            if format.len() != 6 {
                return Err(());
            }

            let red = u8::from_str_radix(&format[0..2], 16).map_err(|_| ())?;
            let green = u8::from_str_radix(&format[2..4], 16).map_err(|_| ())?;
            let blue = u8::from_str_radix(&format[4..], 16).map_err(|_| ())?;

            Screen::set_pixel((x, y), (red, green, blue));
            Ok(&[])
        }
        Some("HELP") => {
            // Send help :'(
            Ok(HELP_MESSAGE)
        }
        Some("SIZE") => {
            // Send screen size
            Ok(Screen::get_screen_size_message())
        }
        _ => {
            // ignored FeelsBadMan
            Ok(&[])
        }
    }
}

fn handle_message_v2(buffer: &[u8]) -> Result<&'static [u8], ()> {
    match buffer.get(0) {
        Some(b'P') | Some(b'p') => parse_px(&buffer[3..]),
        Some(b'S') => {
            if &buffer[1..] == b"IZE" {
                Ok(Screen::get_screen_size_message())
            } else {
                Ok(&[])
            }
        }
        Some(b's') => {
            if &buffer[1..] == b"ize" {
                Ok(Screen::get_screen_size_message())
            } else {
                Ok(&[])
            }
        }
        Some(b'H') => {
            if &buffer[1..] == b"ELP" {
                Ok(HELP_MESSAGE)
            } else {
                Ok(&[])
            }
        }
        Some(b'h') => {
            if &buffer[1..] == b"elp" {
                Ok(HELP_MESSAGE)
            } else {
                Ok(&[])
            }
        }
        _ => Ok(&[]),
    }
}

fn parse_px(buffer: &[u8]) -> Result<&'static [u8], ()> {
    let mut first_index = 0;
    let mut second_index = 0;
    for (i, char) in buffer.iter().enumerate() {
        if char == &b' ' {
            first_index = i;
            break;
        }
    }
    for (i, char) in buffer.iter().enumerate().skip(first_index + 1) {
        if char == &b' ' {
            second_index = i;
            break;
        }
    }
    if second_index == 0 {
        return Err(());
    }

    let x = fast_parse_usize(&buffer[..first_index])?;
    let y = fast_parse_usize(&buffer[first_index + 1..second_index])?;
    let red = fast_parse_hex(&buffer[second_index + 1..second_index + 3])?;
    let green = fast_parse_hex(&buffer[second_index + 3..second_index + 5])?;
    let blue = fast_parse_hex(&buffer[second_index + 5..second_index + 7])?;

    Screen::set_pixel((x, y), (red, green, blue));

    Ok(&[])
}

fn fast_parse_usize(buff: &[u8]) -> Result<usize, ()> {
    let mut result = 0;
    for b in buff {
        let b = *b;
        if b >= b'0' && b <= b'9' {
            result = result * 10 + (b - b'0') as usize;
        } else {
            return Err(());
        }
    }
    Ok(result)
}

#[bench]
fn bench_parse_usize_fast(b: &mut Bencher) {
    let u = b"12345";
    b.iter(|| {
        let u = black_box(u);
        let u = fast_parse_usize(&u[..]).unwrap();
        black_box(u);
    });
}
#[bench]
fn bench_parse_usize_std(b: &mut Bencher) {
    let u = b"12345";
    b.iter(|| {
        let u = black_box(u);
        let str = ::std::str::from_utf8(u).unwrap();
        let u: usize = str.parse().unwrap();
        black_box(u);
    });
}
#[bench]
fn bench_parse_hex_fast(b: &mut Bencher) {
    let u = b"FF";
    b.iter(|| {
        let u = black_box(u);
        let u = fast_parse_hex(&u[..]).unwrap();
        black_box(u);
    });
}
#[bench]
fn bench_parse_hex_std(b: &mut Bencher) {
    let u = b"FF";
    b.iter(|| {
        let u = black_box(u);
        let str = ::std::str::from_utf8(u).unwrap();
        let u = u8::from_str_radix(&str[..], 16).unwrap();
        black_box(u);
    });
}

fn fast_parse_hex(buff: &[u8]) -> Result<u8, ()> {
    let mut result = 0;
    for b in buff {
        let b = *b;
        if b >= b'0' && b <= b'9' {
            result = result * 16 + (b - b'0');
        } else if b >= b'A' && b <= b'F' {
            result = result * 16 + (b - b'A' + 10);
        } else if b >= b'a' && b <= b'f' {
            result = result * 16 + (b - b'a' + 10);
        } else {
            return Err(());
        }
    }
    Ok(result)
}

#[bench]
fn bench_handle_help_message_v1(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"HELP", b"help"];
    b.iter(|| {
        for b in bytes.iter() {
            black_box(handle_message_v1(b).expect("Unexpected error"));
        }
    })
}

#[bench]
fn bench_handle_size_message_v1(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"size", b"SIZE"];
    b.iter(|| {
        for b in bytes.iter() {
            black_box(handle_message_v1(b).expect("Unexpected error"));
        }
    })
}

#[bench]
fn bench_handle_invalid_px_message_v1(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"PX 144 255", b"PX -1 -1 ff0055"];
    b.iter(|| {
        for b in bytes.iter() {
            if let Ok(_) = black_box(handle_message_v1(b)) {
                panic!("Expected error, got Ok: {:?}", ::std::str::from_utf8(&b));
            }
        }
    })
}

#[bench]
fn bench_handle_px_message_v1(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"PX 144 255 FF0055", b"px 144 255 ff0055"];
    b.iter(|| {
        for b in bytes.iter() {
            black_box(handle_message_v1(b).expect("Unexpected error"));
        }
    })
}

#[bench]
fn bench_handle_help_message_v2(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"HELP", b"help"];
    b.iter(|| {
        for b in bytes.iter() {
            black_box(handle_message_v2(b).expect("Unexpected error"));
        }
    })
}

#[bench]
fn bench_handle_size_message_v2(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"size", b"SIZE"];
    b.iter(|| {
        for b in bytes.iter() {
            black_box(handle_message_v2(b).expect("Unexpected error"));
        }
    })
}

#[bench]
fn bench_handle_invalid_px_message_v2(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"PX 144 255", b"px -1 -1 ff0055"];
    b.iter(|| {
        for b in bytes.iter() {
            if let Ok(_) = black_box(handle_message_v2(b)) {
                panic!("Should not be okay");
            }
        }
    })
}

#[bench]
fn bench_handle_px_message_v2(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"PX 144 255 FF0055", b"px 144 255 ff0055"];
    b.iter(|| {
        for b in bytes.iter() {
            black_box(handle_message_v2(b).expect("Unexpected error"));
        }
    });
}
