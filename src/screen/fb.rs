use framebuffer::Framebuffer;

#[derive(Clone)]
pub struct Screen {
    buffer: *mut Framebuffer,
    width: u32,
    height: u32,
}

impl Screen {
    pub fn new() -> Self {
        let buffer = Framebuffer::new("/dev/fb0").expect("Could not open frame buffer");
        let width = buffer.var_screen_info.xres;
        let height = buffer.var_screen_info.yres;
        assert_eq!(
            buffer.var_screen_info.bits_per_pixel, 32,
            "Weird bytes per pixel format (found {}, expected 32), exiting",
            buffer.var_screen_info.bits_per_pixel
        );
        assert_eq!(
            buffer.fix_screen_info.line_length,
            width * 4,
            "Expected a line length of {}, found {}",
            width * 8,
            buffer.fix_screen_info.line_length
        );

        Self {
            buffer: Box::into_raw(Box::new(buffer)),
            width,
            height,
        }
    }
}

unsafe impl Send for Screen {}

impl super::Screen for Screen {
    fn set_pixel(&self, x: u16, y: u16, color: (u8, u8, u8)) {
        if x as u32 >= self.width || y as u32 >= self.height {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        let width = self.width as usize * 4;
        let idx = (x + y * width) * 4;
        let map = &mut unsafe { &mut *self.buffer }.frame;
        if let Some(slice) = map.get_mut(idx..idx + 4) {
            slice[0] = color.0;
            slice[1] = color.1;
            slice[2] = color.2;
        }
    }

    fn running(&self) -> bool {
        true
    }
}

pub struct DummyUpdater;

impl super::ScreenUpdater for DummyUpdater {
    fn update(&mut self) {}
    fn running(&self) -> bool {
        true
    }
}
