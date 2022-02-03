use crate::Args;
use std::convert::Infallible;

cfg_if::cfg_if! {
    if #[cfg(feature = "windowed")] {
        mod windowed;
        pub fn new(args: &Args) -> (impl Screen, Option<impl ScreenUpdater>) {
            windowed::Screen::new(args)
        }
    } else if #[cfg(target_os = "linux")] {
        mod fb;
        pub fn new(args: &Args) -> (impl Screen, Option<impl ScreenUpdater>) {
            fb::Screen::new(args)
        }
    } else {
        compile_error!("Run this on linux to enable framebuffers, or enable the \"windowed\" feature");
    }
}

pub trait Screen: Clone + Send + 'static {
    fn running(&self) -> bool;
    fn set_pixel(&self, x: u16, y: u16, pixel: (u8, u8, u8));
    fn size(&self) -> (u32, u32);
}

pub trait ScreenUpdater {
    fn update(&mut self);
    fn running(&self) -> bool;
}

struct DummyUpdater(Infallible);

impl ScreenUpdater for DummyUpdater {
    fn update(&mut self) {}
    fn running(&self) -> bool {
        true
    }
}
