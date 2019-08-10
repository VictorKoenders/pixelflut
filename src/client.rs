use crate::screen::Screen;
use crate::utils::{parse_hex, parse_usize};
#[cfg(test)]
use std::borrow::Cow;
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

#[test]
fn test_handle_message_response() {
    let _ = Client.handle_message_response(&[
        0x70, 0x0, 0x0, 0x20, 0x36, 0x31, 0x36, 0x31, 0x35, 0x32, 0x38, 0x35, 0x32, 0x38, 0x31,
        0x36, 0x31, 0x35, 0x32, 0x38, 0x31, 0x32, 0x38, 0x31, 0x20,
    ]);

    let _ = Client.handle_message_response(&[
        0x70, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x31, 0x38, 0x34, 0x34, 0x36,
        0x37, 0x34, 0x34, 0x30, 0x37, 0x33, 0x37, 0x30, 0x39, 0x35, 0x35, 0x31, 0x36, 0x31, 0x36,
        0x20, 0x20, 0x20, 0x63, 0x63,
    ]);
}

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
        Some(b'P') | Some(b'p') => {
            if parse_px(buffer.get(3..)).is_none() {
                Err(())
            } else {
                Ok(&[])
            }
        }
        Some(b'S') => {
            if buffer.get(1..) == Some(b"IZE") {
                Ok(Screen::get_screen_size_message())
            } else {
                Ok(&[])
            }
        }
        Some(b's') => {
            if buffer.get(1..) == Some(b"ize") {
                Ok(Screen::get_screen_size_message())
            } else {
                Ok(&[])
            }
        }
        Some(b'H') => {
            if buffer.get(1..) == Some(b"ELP") {
                Ok(HELP_MESSAGE)
            } else {
                Ok(&[])
            }
        }
        Some(b'h') => {
            if buffer.get(1..) == Some(b"elp") {
                Ok(HELP_MESSAGE)
            } else {
                Ok(&[])
            }
        }
        _ => Ok(&[]),
    }
}

#[cfg(test)]
fn handle_message_v3(buffer: &[u8]) -> Option<Cow<'static, [u8]>> {
    use crate::utils::parse_usize_with_len;
    let no_result = Some(Cow::from(Vec::new()));
    // We assume that this is a PX command:
    // 01234567890123456
    // PX X Y RRGGBB
    // PX XX YY RRGGBB
    // PX XXX YYY RRGGBB
    if let Some(mut remaining) = buffer.get(3..) {
        if let Some((x, len)) = parse_usize_with_len(remaining) {
            remaining = &remaining[len + 1..];
            if let Some((y, len)) = parse_usize_with_len(remaining) {
                remaining = &remaining[len + 1..];
                if let (Some(r), Some(g), Some(b)) = (
                    parse_hex(remaining.get(..2)),
                    parse_hex(remaining.get(2..4)),
                    parse_hex(remaining.get(4..6)),
                ) {
                    Screen::set_pixel((x, y), (r, g, b));
                    return no_result;
                }

                if let Some(rgb) = Screen::get_pixel_at(x, y) {
                    return Some(
                        format!(
                            "PX {} {} {:02X}{:02X}{:02X}\r\n",
                            x, y, rgb[0], rgb[1], rgb[2]
                        )
                        .bytes()
                        .collect::<Vec<_>>()
                        .into(),
                    );
                }
            }
        }
    }
    match buffer.get(0) {
        Some(b'S') => {
            if buffer.get(1..) == Some(b"IZE") {
                Some(Screen::get_screen_size_message().into())
            } else {
                no_result
            }
        }
        Some(b's') => {
            if buffer.get(1..) == Some(b"ize") {
                Some(Screen::get_screen_size_message().into())
            } else {
                no_result
            }
        }
        Some(b'H') => {
            if buffer.get(1..) == Some(b"ELP") {
                Some(HELP_MESSAGE.into())
            } else {
                no_result
            }
        }
        Some(b'h') => {
            if buffer.get(1..) == Some(b"elp") {
                Some(HELP_MESSAGE.into())
            } else {
                no_result
            }
        }
        _ => no_result,
    }
}

type PxLocation = ((usize, usize), (u8, u8, u8));

fn parse_px(buffer: Option<&[u8]>) -> Option<PxLocation> {
    let buffer = buffer?;
    let mut iter = buffer.iter();

    let first_index = iter.position(|c| c == &b' ')?;
    let second_index = first_index + iter.position(|c| c == &b' ')?;

    let x = parse_usize(buffer.get(..first_index)?)?;
    let y = parse_usize(buffer.get(first_index + 1..=second_index)?)?;
    let red = parse_hex(buffer.get(second_index + 2..second_index + 4))?;
    let green = parse_hex(buffer.get(second_index + 4..second_index + 6))?;
    let blue = parse_hex(buffer.get(second_index + 6..second_index + 8))?;

    Screen::set_pixel((x, y), (red, green, blue));

    Some(((x, y), (red, green, blue)))
}

#[test]
fn test_parse_px() {
    assert_eq!(
        parse_px(Some(b"1 2 AABBCC")),
        Some(((1, 2), (0xAA, 0xBB, 0xCC)))
    );
    assert_eq!(
        parse_px(Some(b"50 50 FF0000")),
        Some(((50, 50), (0xFF, 0x00, 0x00)))
    );
    assert!(parse_px(None).is_none());
    assert!(parse_px(Some(b"1 2")).is_none());
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
            if black_box(handle_message_v1(b)).is_ok() {
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
    let bytes: &[&'static [u8]] = &[b"PX 144 255 ABCDE", b"PX 144", b"px -1 -1 ff0055"];
    b.iter(|| {
        for _ in 0..100 {
            for b in bytes.iter() {
                if black_box(handle_message_v2(b)).is_ok() {
                    panic!("Should not be okay: {:?}", std::str::from_utf8(b));
                }
            }
        }
    })
}

#[bench]
fn bench_handle_px_message_v2(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"PX 144 255 FF0055", b"px 144 255 ff0055"];
    b.iter(|| {
        for _ in 0..100 {
            for b in bytes.iter() {
                black_box(handle_message_v2(b).expect("Unexpected error"));
            }
        }
    });
}

#[test]
fn test_handle_px_message_v3() {
    crate::utils::initialize_usize();
    crate::utils::initialize_hex();

    let screen = Screen::init();
    let mut random = rand::thread_rng();
    for x in 0..640 {
        for y in 0..480 {
            use rand::Rng;
            let r: u8 = random.gen();
            let g: u8 = random.gen();
            let b: u8 = random.gen();
            let msg = format!("PX {} {} {:02X}{:02X}{:02X}", x, y, r, g, b);
            let result = handle_message_v3(msg.as_bytes()).expect("Expected Some");
            if !result.is_empty() {
                panic!("Invalid response: {:?}", std::str::from_utf8(&result));
            }

            let color = Screen::get_pixel_at(x, y).expect("Could not get pixel");
            assert_eq!(b, color[0]);
            assert_eq!(g, color[1]);
            assert_eq!(r, color[2]);
        }
    }
}

#[bench]
fn bench_handle_px_message_v3(b: &mut Bencher) {
    let bytes: &[&'static [u8]] = &[b"PX 144 255 FF0055", b"px 144 255 ff0055"];
    b.iter(|| {
        for _ in 0..100 {
            for b in bytes.iter() {
                black_box(handle_message_v3(b).expect("Unexpected error"));
            }
        }
    });
}
