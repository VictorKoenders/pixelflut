mod parse;

use crate::screen::Screen;

pub struct ClientState {
    buffer: [u8; 1000],
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            buffer: [0u8; 1000],
        }
    }
}

impl ClientState {
    pub fn recv_buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    pub fn parse_buffer(&mut self, screen: &impl Screen, len: usize) -> Option<ResponseMessage> {
        let mut slice = &self.buffer[..len];
        loop {
            if matches!(slice.get(0..4), Some(b"help" | b"HELP")) {
                return Some(ResponseMessage::Help);
            }
            println!("{:?}", std::str::from_utf8(&slice[..(10.min(slice.len()))]));
            let (x, remaining) = parse::parse_coordinate(slice)?;
            println!("x = {}", x);
            println!(
                "{:?}",
                std::str::from_utf8(&remaining[..(10.min(remaining.len()))])
            );
            let (y, remaining) = parse::parse_coordinate(remaining)?;
            println!("y = {}", y);
            println!(
                "{:?}",
                std::str::from_utf8(&remaining[..(10.min(remaining.len()))])
            );
            let color = parse::parse_color(remaining)?;
            println!("x: {}, y: {}, color: {:?}", x, y, color);
            screen.set_pixel(x, y, color);
            slice = &remaining[6..];
            while matches!(slice.get(0), Some(&b'\r' | &b'\n')) {
                slice = &slice[1..];
            }
        }
    }
}

pub enum ResponseMessage {
    Help,
}

impl ResponseMessage {
    pub fn into_bytes(self) -> &'static [u8] {
        match self {
            ResponseMessage::Help => HELP_MESSAGE,
        }
    }
}

const HELP_MESSAGE: &[u8] = br#"Possible commands:
- PX <x> <y> <rgbhex>: Set the pixel at X/Y the given RGB value. This needs to be a 6-character HEX value, e.g. 000000 for black and FFFFFF for white
- SIZE: returns the size of the screen, in pixels
- HELP: returns this help
"#;
