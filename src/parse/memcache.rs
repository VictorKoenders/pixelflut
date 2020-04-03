use crate::config::Config;
use crate::output::ScreenBuffer;
use byteorder::{BigEndian, ByteOrder};
use std::borrow::Cow;

const HELP_TEXT: &str = r"Available commands:
PX x y RRGGBB - Sets the given pixel at x/y to the given HEX code
Examples:
PX 10 10 FF0000    Sets the pixel at 10/10 to red
PX 20 20 00FF00    Sets the pixel at 20/20 to green

HELP - Sends this helptext
SIZE - Get the size of the screen
";

const MEMCACHE_BUFFER_SIZE: usize = 10000;
pub struct MemCache {
    integer_cache: IntegerCache,
    hex_cache: HexCache,
    size_message: &'static str,
    buffer: [u8; MEMCACHE_BUFFER_SIZE],
}

unsafe impl Send for MemCache {}

impl super::Parse for MemCache {
    fn new(config: Config) -> Self {
        let size_message = Box::leak(
            format!(
                "SIZE {} {}\n",
                config.screen_dimensions.0, config.screen_dimensions.1
            )
            .into_boxed_str(),
        );
        Self {
            hex_cache: HexCache::new(),
            integer_cache: IntegerCache::new(
                config.screen_dimensions.0.max(config.screen_dimensions.1),
            ),
            size_message,
            buffer: [0u8; MEMCACHE_BUFFER_SIZE],
        }
    }
    fn write_buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
    fn parse<S: ScreenBuffer>(&mut self, count: usize, screen: &S) -> Option<Cow<'static, str>> {
        let mut buffer = &self.buffer[..count];
        loop {
            // Min PXpacket size is 14:
            // PX x y RRGGBB\n
            if starts_with(&buffer, b"px ", b"PX ") {
                buffer = &buffer[3..];
                match self.parse_pixel(screen, &buffer) {
                    Some(len) => {
                        buffer = &buffer[len..];
                        while !buffer.is_empty() && !is_alphanumeric(buffer[0]) {
                            buffer = &buffer[1..];
                        }
                    }
                    None => break None,
                }
            } else if starts_with(&buffer, b"size", b"SIZE") {
                break Some(Cow::from(self.size_message));
            } else if starts_with(&buffer, b"help", b"HELP") {
                break Some(Cow::from(HELP_TEXT));
            } else if let Some(index) = buffer.iter().position(|&c| c == b'\n') {
                buffer = &buffer[index + 1..];
            } else {
                break None;
            }
        }
    }
}

#[test]
fn test_fuzz_data() {
    use crate::output::{Fake, Screen};
    use crate::parse::Parse;

    for data in &[
        &[0x58, 0x50, 0x15, 0x20, 0xad, 0x30][..],
        &b"PX 1 1 FF0E00 8 8 \nPX 2 0 11"[..],
        &[
            0x44, 0x30, 0x16, 0x0a, 0x6c, 0x58, 0x20, 0x38, 0x20, 0x32, 0x20, 0x32, 0x20, 0x20,
            0x35, 0x20, 0x35, 0x30, 0x30, 0x05, 0xff, 0xff, 0x20, 0x33, 0x20, 0xff, 0xff, 0xff,
            0x00, 0x00, 0x80, 0x00, 0x02, 0x00, 0x00, 0x3a, 0xff, 0xff, 0x20, 0x33, 0x20, 0x33,
            0xff, 0xff, 0x00, 0x00, 0x80, 0x00, 0x02, 0x00, 0x00, 0x3a, 0x31, 0x30, 0x46, 0x30,
            0x30, 0x30, 0x05, 0xff, 0x7f, 0x05, 0x20, 0x46, 0x20, 0x38, 0x13, 0x38, 0x2c, 0x0a,
            0x50, 0x58,
        ][..],
    ] {
        let mut config = Config::default();
        config.screen_dimensions = (10, 10);
        let screen = Fake::create(config.clone());
        let mut memcache = MemCache::new(config);
        memcache.write_buffer()[..data.len()].copy_from_slice(data);
        memcache.parse(data.len(), &screen);
    }
}
fn starts_with(buffer: &[u8], lowercase: &[u8], uppercase: &[u8]) -> bool {
    debug_assert_eq!(lowercase.len(), uppercase.len());
    let len = lowercase.len();
    if buffer.len() < len {
        return false;
    }
    for i in 0..len {
        if buffer[i] != lowercase[i] && buffer[i] != uppercase[i] {
            return false;
        }
    }
    true
}

fn is_alphanumeric(c: u8) -> bool {
    (c >= b'0' && c <= b'9') || (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z')
}

impl MemCache {
    fn parse_pixel<S: ScreenBuffer>(&self, screen: &S, buffer: &[u8]) -> Option<usize> {
        let (x, x_len) = self.integer_cache.lookup(&buffer)?;
        let buffer = &buffer[x_len + 1..];
        let (y, y_len) = self.integer_cache.lookup(&buffer)?;
        let buffer = &buffer[y_len + 1..];

        if buffer.len() < 6 {
            return None;
        }

        let color_r = self.hex_cache.lookup(&buffer[..2])?;
        let color_g = self.hex_cache.lookup(&buffer[2..4])?;
        let color_b = self.hex_cache.lookup(&buffer[4..6])?;

        let color = (color_r, color_g, color_b);
        screen.set_pixel(x, y, color);
        // x, space, y, space, rrggbb
        // x_len, 1, y_len, 1, 6
        Some(x_len + 1 + y_len + 1 + 6)
    }
}

impl Clone for MemCache {
    fn clone(&self) -> Self {
        Self {
            integer_cache: self.integer_cache.clone(),
            hex_cache: self.hex_cache.clone(),
            size_message: self.size_message,
            buffer: [0u8; MEMCACHE_BUFFER_SIZE],
        }
    }
}

#[derive(Clone)]
struct IntegerCache {
    ptr: *const u16,
    len: u32,
}

impl IntegerCache {
    fn new(max: u16) -> Self {
        println!("Building integer cache, this might take a while");
        let start = std::time::Instant::now();

        let max_len = Self::calculate_number(max.to_string().as_bytes());
        let mut vec = vec![u16::max_value(); max_len as usize + 1];
        for num in (0..=max).rev() {
            let str = num.to_string();
            let number = Self::calculate_number(str.as_bytes());
            vec[number as usize] = num;
        }
        let ptr = vec.as_ptr();
        let len = vec.len() as u32;
        std::mem::forget(vec);
        println!("Integer cache: {}kb", len / 1024 * 2);
        println!("Took {}ms", start.elapsed().as_micros() as f32 / 1000.);

        Self { ptr, len }
    }

    fn lookup(&self, buff: &[u8]) -> Option<(u16, usize)> {
        for len in (1..=4).rev() {
            if len >= buff.len() {
                return None;
            }
            let num = Self::calculate_number(&buff[..len]);
            if num < self.len {
                let val = unsafe { std::ptr::read(self.ptr.offset(num as isize)) };
                if val != u16::max_value() {
                    return Some((val, len));
                }
            }
        }
        None
    }

    fn calculate_number(buff: &[u8]) -> u32 {
        debug_assert!(!buff.is_empty() && buff.len() <= 4);
        let mut buffer = [0x30; 4];
        let offset = 4 - buff.len();
        buffer[offset..].copy_from_slice(buff);

        // 0x30303030 is the text "0000"
        BigEndian::read_u32(&buffer).wrapping_sub(0x3030_3030)
    }
}

#[derive(Clone)]
struct HexCache {
    ptr: *const u16,
    len: u16,
}

impl HexCache {
    fn new() -> Self {
        println!("Building hex cache, this might take a while");
        let start = std::time::Instant::now();
        let max_len = Self::calculate_number(&b"ff"[..]);

        let mut vec = vec![u16::max_value(); max_len as usize + 1];
        for num in (0..=u8::max_value()).rev() {
            for str in &[format!("{:02x}", num), format!("{:02X}", num)] {
                let number = Self::calculate_number(str.as_bytes());
                vec[number as usize] = num as u16;
            }
        }
        let ptr = vec.as_ptr();
        let len = vec.len() as u16;
        std::mem::forget(vec);
        println!("Hex cache: {}kb", len / 1024 * 2);
        println!("Took {}ms", start.elapsed().as_micros() as f32 / 1000.);

        Self { ptr, len }
    }

    fn lookup(&self, buff: &[u8]) -> Option<u8> {
        let num = Self::calculate_number(buff);
        if num >= self.len {
            None
        } else {
            let val = unsafe { std::ptr::read(self.ptr.offset(num as isize)) };
            if val == u16::max_value() {
                None
            } else {
                Some(val as u8)
            }
        }
    }

    fn calculate_number(buff: &[u8]) -> u16 {
        debug_assert!(buff.len() == 2);
        let val = BigEndian::read_u16(buff);
        // 0x4141 is the text "00"
        val.wrapping_sub(0x3030)
    }
}
