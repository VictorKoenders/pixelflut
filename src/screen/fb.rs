use framebuffer::Framebuffer;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[derive(Clone)]
pub struct Screen {
    buffer: *mut Framebuffer,
    width: u32,
    height: u32,
    line_length: u32,
    bytes_per_pixel: u32,
    write_count: Arc<AtomicUsize>,
}

impl Screen {
    pub fn new() -> Self {
        let buffer = Framebuffer::new("/dev/fb0").expect("Could not open frame buffer");
        let width = buffer.var_screen_info.xres;
        let height = buffer.var_screen_info.yres;
        let line_length = buffer.fix_screen_info.line_length;
        let bytes_per_pixel = buffer.var_screen_info.bits_per_pixel / 8;
        println!("Screen is {}x{} pixels", width, height);
        println!(
            "Line length: {}, bytes per pixel: {}",
            line_length, bytes_per_pixel
        );

        Self {
            buffer: Box::into_raw(Box::new(buffer)),
            width,
            height,
            line_length,
            bytes_per_pixel,
            write_count: Arc::new(AtomicUsize::new(0)),
        }
    }
}

unsafe impl Send for Screen {}

impl super::Screen for Screen {
    fn set_pixel(&self, x: u16, y: u16, (r, g, b): (u8, u8, u8)) {
        let x = x as u32;
        let y = y as u32;
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = (x * self.bytes_per_pixel + y * self.line_length) as usize;
        let map = &mut unsafe { &mut *self.buffer }.frame;
        if cfg!(debug_assertions) {
            assert!(
                map.get_mut(idx..idx + 4).is_some(),
                "Invalid idx for {}/{} (width: {}, height: {})",
                x,
                y,
                self.width,
                self.height
            );
        }
        let slice = unsafe { map.get_unchecked_mut(idx..idx + 4) };
        slice[0] = b;
        slice[1] = g;
        slice[2] = r;

        let count = self.write_count.fetch_add(1, Ordering::Relaxed);
        if count % 1_000_000 == 0 {
            println!("Wrote {} pixels", count);
        }
    }

    fn running(&self) -> bool {
        true
    }

    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

pub struct DummyUpdater;

impl super::ScreenUpdater for DummyUpdater {
    fn update(&mut self) {}
    fn running(&self) -> bool {
        true
    }
}
