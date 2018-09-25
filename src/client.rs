use screen::Screen;
use std::io::Write;
use std::net::TcpStream;
use std::str;
#[cfg(test)]
use test::{black_box, Bencher};
use Result;

const HELP_MESSAGE: &[u8] = br#"Possible commands:
- PX <x> <y> <rgbhex>: Set the pixel at X/Y the given RGB value. This needs to be a 6-character HEX value, e.g. 000000 for black and FFFFFF for white
- SIZE: returns the size of the screen, in pixels
- HELP: returns this help
"#;

pub struct Client;

impl Client {
    pub fn handle_message(&self, stream: &mut TcpStream, buffer: &[u8]) -> Result<()> {
        let slice = handle_message_v1(buffer)?;
        if !slice.is_empty() {
            stream.write_all(slice)?;
        }
        Ok(())
        /*
        let str = str::from_utf8(&buffer)?;
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
                Screen::set_pixel((x, y), (r, g, b));
            }
            Some("HELP") => {
                // Send help :'(
                stream.write_all(HELP_MESSAGE.as_bytes())?;
            }
            Some("SIZE") => {
                // Send screen size
                stream.write_all(Screen::get_screen_size_message())?;
            }
            _ => { /* ignored FeelsBadMan */ }
        }
        Ok(())
        */
    }
}

fn handle_message_v1(buffer: &[u8]) -> Result<&'static [u8]> {
    let str = str::from_utf8(&buffer)?;
    let mut iter = str.trim().split(' ');

    match iter.next() {
        Some("PX") => {
            // Set pixel
            macro_rules! unwrap_or_return {
                ($stmt:expr) => {
                    match $stmt {
                        Some(n) => n,
                        None => return Ok(&[]),
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
            Screen::set_pixel((x, y), (r, g, b));
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

#[cfg(test)]
fn handle_message_v2(buffer: &[u8]) -> Result<&'static [u8]> {
    match buffer.get(0) {
        Some(b'P') | Some(b'p') => parse_px(&buffer[3..]),
        Some(b'S') => if buffer == b"SIZE" {
            Ok(Screen::get_screen_size_message())
        } else {
            Ok(&[])
        },
        Some(b's') => if buffer == b"size" {
            Ok(Screen::get_screen_size_message())
        } else {
            Ok(&[])
        },
        Some(b'H') => if buffer == b"HELP" {
            Ok(HELP_MESSAGE)
        } else {
            Ok(&[])
        },
        Some(b'h') => if buffer == b"help" {
            Ok(HELP_MESSAGE)
        } else {
            Ok(&[])
        },
        _ => Ok(&[]),
    }
}

#[cfg(test)]
fn parse_px(buffer: &[u8]) -> Result<&'static [u8]> {
    let mut first_index = 0;
    let mut second_index = 0;
    for i in 0..buffer.len() {
        if buffer[i] == b' ' {
            first_index = i;
            break;
        }
    }
    for i in first_index + 1..buffer.len() {
        if buffer[i] == b' ' {
            second_index = i;
            break;
        }
    }
    assert_eq!(3, first_index);
    assert_eq!(7, second_index);

    use std::str::from_utf8;

    assert_eq!(Ok("144"), from_utf8(&buffer[..first_index]));
    assert_eq!(Ok("255"), from_utf8(&buffer[first_index + 1..second_index]));
    assert_eq!(Ok("FF0055"), from_utf8(&buffer[second_index + 1..]));
    Ok(&[])
}

#[bench]
fn bench_handle_message_v1(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"PX 144 255 FF0055", b"HELP", b"SIZE"];
    b.iter(|| {
        for b in bytes.iter() {
            black_box(handle_message_v1(b).expect("Unexpected error"));
        }
    })
}

#[bench]
fn bench_handle_message_v2(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"PX 144 255 FF0055", b"HELP", b"SIZE"];
    b.iter(|| {
        for b in bytes.iter() {
            black_box(handle_message_v2(b).expect("Unexpected error"));
        }
    })
}
