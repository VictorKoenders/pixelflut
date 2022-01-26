use framebuffer::Framebuffer;

struct ScreenInner {
    buffer: Framebuffer,
    width: u32,
    height: u32,
    line_length: u32,
    bytes_per_pixel: u32,
}

#[derive(Clone)]
pub struct Screen {
    inner: *mut ScreenInner,
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
            inner: Box::into_raw(Box::new(ScreenInner {
                buffer,
                width,
                height,
                line_length,
                bytes_per_pixel,
            })),
        }
    }
}

unsafe impl Send for Screen {}

impl super::Screen for Screen {
    fn set_pixel(&self, x: u16, y: u16, (r, g, b): (u8, u8, u8)) {
        let inner = unsafe { &mut *self.inner };
        let x = x as u32;
        let y = y as u32;
        if x >= inner.width || y >= inner.height {
            return;
        }
        let idx = (x * inner.bytes_per_pixel + y * inner.line_length) as usize;
        let map = &mut inner.buffer.frame;
        if cfg!(debug_assertions) {
            assert!(
                map.get_mut(idx..idx + 4).is_some(),
                "Invalid idx for {}/{} (width: {}, height: {})",
                x,
                y,
                inner.width,
                inner.height
            );
        }
        let slice = unsafe { map.get_unchecked_mut(idx..idx + 4) };
        slice[0] = b;
        slice[1] = g;
        slice[2] = r;
    }

    fn running(&self) -> bool {
        true
    }

    fn size(&self) -> (u32, u32) {
        let inner = unsafe { &*self.inner };
        (inner.width, inner.height)
    }
}
