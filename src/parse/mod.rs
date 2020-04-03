#[cfg(feature = "memcache")]
mod memcache;
#[cfg(feature = "memcache")]
pub use self::memcache::MemCache;

use crate::config::Config;
use crate::output::ScreenBuffer;
use std::borrow::Cow;

pub trait Parse: Clone + Send + 'static {
    fn new(config: Config) -> Self;
    fn write_buffer(&mut self) -> &mut [u8];
    fn parse<S: ScreenBuffer>(&mut self, count: usize, screen: &S) -> Option<Cow<'static, str>>;
}
