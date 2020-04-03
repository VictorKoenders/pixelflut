mod max_threads;
pub use self::max_threads::MaxThreads;

#[cfg(feature = "tokio")]
mod tokio;
#[cfg(feature = "tokio")]
pub use self::tokio::Tokio;

use crate::config::Config;
use crate::output::ScreenBuffer;
use crate::parse::Parse;

pub trait Input {
    fn spawn<P: Parse + 'static, S: ScreenBuffer + 'static>(
        config: Config,
        parse: P,
        buffer: S,
    ) -> Self;
}
