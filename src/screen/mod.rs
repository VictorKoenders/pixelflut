cfg_if::cfg_if! {
    if #[cfg(feature = "windowed")] {
        mod windowed;
        pub fn new() -> (impl Screen, Option<impl ScreenUpdater>) {
            windowed::Screen::new()
        }

    } else {
        mod fb;
    }
}

pub trait Screen: Clone + Send + 'static {
    fn running(&self) -> bool;
    fn set_pixel(&self, x: u16, y: u16, pixel: (u8, u8, u8));
}

pub trait ScreenUpdater {
    fn update(&mut self);
    fn running(&self) -> bool;
}
