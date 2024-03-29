// Measured to be about 25% slower
#![allow(clippy::manual_range_contains)]

pub fn parse_coordinate(buffer: &[u8]) -> Option<(u16, &[u8])> {
    let mut result = 0;

    for (idx, b) in buffer.iter().copied().enumerate() {
        if b >= b'0' && b <= b'9' {
            result = result * 10 + (b - b'0') as u16;
        } else if b == b' ' && idx != 0 {
            return Some((
                result,
                // Safety: We know that `idx` is a space so `idx+1..` will always exist
                unsafe { buffer.get_unchecked(idx + 1..) },
            ));
        } else {
            return None;
        }

        if result > super::MAX_VALID_NUMBER {
            return None;
        }
    }
    None
}

pub fn parse_color(buff: &[u8]) -> Option<(u8, u8, u8)> {
    let slice = buff.get(0..6)?;
    let r = parse_hex(slice[0..2].try_into().unwrap())?;
    let g = parse_hex(slice[2..4].try_into().unwrap())?;
    let b = parse_hex(slice[4..6].try_into().unwrap())?;
    Some((r, g, b))
}

fn parse_hex(buff: &[u8; 2]) -> Option<u8> {
    let mut result = 0;
    for b in buff.iter().copied() {
        if b >= b'0' && b <= b'9' {
            result = result * 16 + (b - b'0');
        } else if b >= b'A' && b <= b'F' {
            result = result * 16 + (b - b'A' + 10);
        } else if b >= b'a' && b <= b'f' {
            result = result * 16 + (b - b'a' + 10);
        } else {
            return None;
        }
    }
    Some(result)
}
pub fn parse_color_unwrapped(buff: &[u8]) -> Option<(u8, u8, u8)> {
    let slice = buff.get(0..6)?;
    let r = parse_hex_unwrapped(slice[0..2].try_into().unwrap())?;
    let g = parse_hex_unwrapped(slice[2..4].try_into().unwrap())?;
    let b = parse_hex_unwrapped(slice[4..6].try_into().unwrap())?;
    Some((r, g, b))
}

fn parse_hex_unwrapped(buff: &[u8; 2]) -> Option<u8> {
    fn get_hex(b: u8) -> Option<u8> {
        Some(
            b - if b >= b'0' && b <= b'9' {
                b'0'
            } else if b >= b'A' && b <= b'F' {
                b'A' - 10
            } else if b >= b'a' && b <= b'f' {
                b'a' - 10
            } else {
                return None;
            },
        )
    }

    let [first, second] = buff;

    let first = get_hex(*first)?;
    let second = get_hex(*second)?;

    Some(first * 16 + second)
}

pub fn initialize() {
    // do nothing
}

#[test]
fn test() {
    assert_eq!(
        parse_coordinate(b"1920 "),
        Some((1920u16, Default::default()))
    );
    assert_eq!(parse_color_unwrapped(b"df2B25"), Some((0xdf, 0x2B, 0x25)));
    assert_eq!(parse_color_unwrapped(b"9347ff"), Some((0x93, 0x47, 0xff)));
}
