use super::*;
use crate::config::Config;
use ::minifb::{Key, Window, WindowOptions};

pub struct MiniFb {
    buffer: MiniFbBuffer,
    window: Window,
}

impl Screen for MiniFb {
    type Buffer = MiniFbBuffer;
    fn create(config: Config) -> Self {
        let (width, height) = config.screen_dimensions;
        let mut window = Window::new(
            "Pixelflut ",
            width as _,
            height as _,
            WindowOptions::default(),
        )
        .expect("Could not create a minifb window");
        window.limit_update_rate(Some(std::time::Duration::from_millis(1000 / 60)));
        let buffer = MiniFbBuffer::new(width, height);
        Self { buffer, window }
    }
    fn get_buffer(&self) -> Self::Buffer {
        self.buffer.clone()
    }
    fn draw(&mut self) {
        self.window
            .update_with_buffer(
                self.buffer.get_image(),
                self.buffer.size.0 as _,
                self.buffer.size.1 as _,
            )
            .unwrap();
    }
    fn is_running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }
}

#[derive(Clone)]
pub struct MiniFbBuffer {
    buffer: *mut u32,
    size: (u16, u16),
}

impl MiniFbBuffer {
    fn new(width: u16, height: u16) -> Self {
        let mut buffer = vec![0; width as usize * height as usize];
        let ptr = buffer.as_mut_ptr();
        std::mem::forget(buffer);
        Self {
            buffer: ptr,
            size: (width, height),
        }
    }
    fn get_image(&self) -> &[u32] {
        unsafe {
            let size = self.size.0 as usize * self.size.1 as usize;
            std::slice::from_raw_parts(self.buffer, size)
        }
    }
}
unsafe impl Send for MiniFbBuffer {}
unsafe impl Sync for MiniFbBuffer {}

impl ScreenBuffer for MiniFbBuffer {
    fn set_pixel(&self, x: u16, y: u16, color: (u8, u8, u8)) {
        if x >= self.size.0 || y >= self.size.1 {
            return;
        }
        let index = calculate_index(x, y, self.size);
        unsafe {
            std::ptr::write(
                self.buffer.offset(index),
                from_u8_rgb(color.0, color.1, color.2),
            )
        };
    }

    fn get_pixel(&self, x: u16, y: u16) -> (u8, u8, u8) {
        if x >= self.size.0 || y >= self.size.1 {
            return (0, 0, 0);
        }

        let index = calculate_index(x, y, self.size);
        let val = unsafe { std::ptr::read(self.buffer.offset(index)) };
        to_u8_rgb(val)
    }
}

const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

const fn to_u8_rgb(val: u32) -> (u8, u8, u8) {
    let b = val & 0xff;
    let g = (val >> 8) & 0xff;
    let r = (val >> 16) & 0xff;
    (r as u8, b as u8, g as u8)
}

const fn calculate_index(x: u16, y: u16, size: (u16, u16)) -> isize {
    (x as isize) + (y as isize) * (size.0 as isize)
}
