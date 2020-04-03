#[cfg(feature = "minifb")]
mod minifb;
#[cfg(feature = "minifb")]
pub use self::minifb::MiniFb;

use crate::config::Config;

pub trait Screen {
    type Buffer: ScreenBuffer;
    fn create(config: Config) -> Self;
    fn get_buffer(&self) -> Self::Buffer;
    fn draw(&mut self);
    fn is_running(&self) -> bool {
        true
    }
}

pub trait ScreenBuffer: Clone + Send + Sync + 'static {
    fn set_pixel(&self, x: u16, y: u16, color: (u8, u8, u8));
    fn get_pixel(&self, x: u16, y: u16) -> (u8, u8, u8);
}

#[derive(Clone)]
pub struct Fake;

impl Screen for Fake {
    type Buffer = Self;
    fn create(_: Config) -> Self {
        Self
    }
    fn get_buffer(&self) -> Self {
        Self
    }
    fn draw(&mut self) {}
    fn is_running(&self) -> bool {
        false
    }
}

impl ScreenBuffer for Fake {
    fn set_pixel(&self, _x: u16, _y: u16, _color: (u8, u8, u8)) {}
    fn get_pixel(&self, _x: u16, _y: u16) -> (u8, u8, u8) {
        (0, 0, 0)
    }
}
