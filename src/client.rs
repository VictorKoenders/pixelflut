use screen::Screen;
use std::io::Write;
use std::net::TcpStream;
use std::str;
use Result;

const HELP_MESSAGE: &str = r#"Possible commands:
- PX <x> <y> <rgbhex>: Set the pixel at X/Y the given RGB value. This needs to be a 6-character HEX value, e.g. 000000 for black and FFFFFF for white
- SIZE: returns the size of the screen, in pixels
- HELP: returns this help
"#;

pub struct Client;

impl Client {
    pub fn handle_message(&self, stream: &mut TcpStream, buffer: &[u8]) -> Result<()> {
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
    }
}
