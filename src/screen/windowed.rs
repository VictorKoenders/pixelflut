use minifb::{Key, Window, WindowOptions};
use std::{alloc::Layout, net::TcpStream, ptr::NonNull};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

#[derive(Clone)]
pub struct Screen {
    buffer: NonNull<[u32; WIDTH * HEIGHT]>,
}

// Safe because we're only setting integer on our `buffer: *mut` pointer, and not re-allocating it or something
unsafe impl Send for Screen {}

impl Screen {
    pub fn new() -> (Self, Option<ScreenUpdater>) {
        let buffer = {
            let layout = Layout::array::<u32>(WIDTH * HEIGHT).unwrap();
            NonNull::new(unsafe { std::alloc::alloc(layout).cast() })
                .expect("Could not allocate screen buffer")
        };
        let screen = Self { buffer };

        let mut updater = ScreenUpdater {
            buffer,
            window: Window::new(
                "Pixelflut",
                WIDTH,
                HEIGHT,
                WindowOptions {
                    resize: false,
                    ..Default::default()
                },
            )
            .unwrap(),
            running: true,
        };

        // Limit to max ~60 fps update rate
        updater
            .window
            .limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        super::ScreenUpdater::update(&mut updater);

        (screen, Some(updater))
    }
}

impl super::Screen for Screen {
    fn running(&self) -> bool {
        true
    }

    fn set_pixel(&self, x: u16, y: u16, pixel: (u8, u8, u8)) {
        let x = x as usize;
        let y = y as usize;
        let idx = x + y * WIDTH;
        let slice = unsafe { &mut *self.buffer.as_ptr() };
        if let Some(old) = slice.get_mut(idx) {
            *old = (pixel.0 as u32) << 16 | (pixel.1 as u32) << 8 | (pixel.2 as u32);
        }
    }

    fn size(&self) -> (u32, u32) {
        (WIDTH as u32, HEIGHT as u32)
    }
}

pub struct ScreenUpdater {
    buffer: NonNull<[u32; WIDTH * HEIGHT]>,
    window: Window,
    running: bool,
}

impl super::ScreenUpdater for ScreenUpdater {
    fn update(&mut self) {
        let buffer: &[u32; WIDTH * HEIGHT] = unsafe { self.buffer.as_ref() };
        self.window
            .update_with_buffer(buffer, WIDTH, HEIGHT)
            .expect("Could not update screen");

        if !self.window.is_open() || self.window.is_key_down(Key::Escape) {
            self.running = false;
            // connect to the server so that we can shut that down correctly
            let _ = TcpStream::connect("localhost:1234");
        }
    }

    fn running(&self) -> bool {
        self.running
    }
}
