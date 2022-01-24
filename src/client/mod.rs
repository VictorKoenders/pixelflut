use crate::{
    parse::{parse_color, parse_coordinate},
    screen::Screen,
};
use std::convert::TryInto;

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
        while let Some(prefix) = slice.get(0..3) {
            let arr: &[u8; 3] = unsafe { prefix.try_into().unwrap_unchecked() };

            if arr == b"px " || arr == b"PX " {
                let remaining = unsafe { slice.get_unchecked(3..) };
                let (x, remaining) = parse_coordinate(remaining)?;
                let (y, remaining) = parse_coordinate(remaining)?;
                let color = parse_color(remaining)?;
                screen.set_pixel(x, y, color);
                slice = &remaining[6..];
                while matches!(slice.get(0), Some(&b'\r' | &b'\n')) {
                    slice = &slice[1..];
                }
            } else {
                #[cold]
                fn non_px_path(
                    prefix: &[u8; 3],
                    (width, height): (u32, u32),
                ) -> Option<ResponseMessage> {
                    match prefix {
                        b"hel" | b"HEL" => Some(ResponseMessage::Help),
                        b"siz" | b"SIZ" => Some(ResponseMessage::Size { width, height }),
                        _ => None,
                    }
                }

                if let Some(msg) = non_px_path(arr, screen.size()) {
                    return Some(msg);
                }
                if let Some(idx) = slice.iter().position(|c| c == &b'\n') {
                    slice = unsafe { slice.get_unchecked(idx + 1..) };
                } else {
                    break;
                }
            }
        }
        None
    }
}

pub enum ResponseMessage {
    Help,
    Size { width: u32, height: u32 },
}

impl ResponseMessage {
    pub fn as_bytes(&self) -> std::borrow::Cow<'static, [u8]> {
        match self {
            ResponseMessage::Help => HELP_MESSAGE.into(),
            ResponseMessage::Size { width, height } => {
                format!("SIZE {} {}\r\n", width, height).into_bytes().into()
            }
        }
    }
}

const HELP_MESSAGE: &[u8] = br#"Possible commands:
- PX <x> <y> <rgbhex>: Set the pixel at X/Y the given RGB value. This needs to be a 6-character HEX value, e.g. 000000 for black and FFFFFF for white
- SIZE: returns the size of the screen, in pixels
- HELP: returns this help
"#;
