#[cfg(not(test))]
use framebuffer::Framebuffer;
use std::cell::UnsafeCell;

#[cfg(test)]
pub struct Screen {
    buffer: UnsafeCell<Vec<u8>>,
    width: usize,
    height: usize,
    frame_size_message: Vec<u8>
}

#[cfg(test)]
impl Screen {
    pub fn init() -> Screen {
        crate::utils::initialize_all();
        Screen {
            buffer: UnsafeCell::new(vec![0u8; 800 * 600 * 3]),
            width: 800,
            height: 600,
            frame_size_message: Vec::new()
        }
    }

    fn slice(&self) -> &[u8] {
        unsafe{ &*self.buffer.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn slice_mut(&self) -> &mut [u8] {
        unsafe{ &mut *self.buffer.get() }
    }

    fn height(&self) -> usize { self.height }
    fn width(&self) -> usize { self.width }

    fn line_length(&self) -> usize {
        self.width * 3
    }

    fn bytes_per_pixel(&self) -> usize {
        3
    }
}

#[cfg(not(test))]
pub struct Screen {
    buffer: UnsafeCell<Framebuffer>,
    frame_size_message: Vec<u8>
}

#[cfg(not(test))]
impl Screen {
    pub fn init() -> Screen {
        crate::utils::initialize_all();
        let buffer = Framebuffer::new("/dev/fb0").expect("Could not open frame buffer");

        let width = buffer.var_screen_info.xres;
        let height = buffer.var_screen_info.yres;
        let screen = Screen {
            buffer: UnsafeCell::new(buffer),
            frame_size_message: format!("SIZE {} {}\n", width, height)
                .as_bytes()
                .into(),
        };
        println!("Width: {}, height: {}", screen.width(), screen.height());
        screen
    }

    #[allow(clippy::mut_from_ref)]
    fn slice_mut(&self) -> &mut [u8] {
        unsafe { (&mut *self.buffer.get()).frame.as_mut_slice() }
    }

    fn slice(&self) -> &[u8] {
        unsafe { (&mut *self.buffer.get()).frame.as_slice() }
    }

    fn width(&self) -> usize {
        unsafe { (&*self.buffer.get()).var_screen_info.xres as usize }
    }

    fn height(&self) -> usize {
        unsafe { (&*self.buffer.get()).var_screen_info.yres as usize }
    }

    fn line_length(&self) -> usize {
        unsafe { (&*self.buffer.get()).fix_screen_info.line_length as usize }
    }

    fn bytes_per_pixel(&self) -> usize {
        unsafe { (&*self.buffer.get()).var_screen_info.bits_per_pixel as usize / 8 }
    }

}

unsafe impl Send for Screen {}
unsafe impl Sync for Screen {}

impl Screen {
    pub fn get_pixel_at(&self, x: usize, y: usize) -> Option<[u8; 3]> {
        if x >= self.width() || y >= self.height() {
            None
        } else {
            let start_index = (y * self.line_length() + x * self.bytes_per_pixel()) as usize;
            let slice = self.slice();
            debug_assert!(slice.len() > start_index + 2);
            Some([
                slice[start_index],
                slice[start_index + 1],
                slice[start_index + 2],
            ])
        }
    }

    #[cfg(test)]
    pub fn all(&self, color: [u8; 3]) -> bool {
        let slice = self.slice();
        for (i, chunk) in slice.chunks(3).enumerate() {
            if chunk != &color {
                println!(
                    "position {} does not match: {:?} (expected {:?})",
                    i, chunk, color
                );
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    pub fn set_pixel_v1(&self, (x, y): (usize, usize), (red, green, blue): (u8, u8, u8)) {
        if x >= self.width() || y >= self.height() {
            return;
        }
        let start_index = y * self.line_length() + x * self.bytes_per_pixel();
        let slice = self.slice_mut();
        slice[start_index] = blue;
        slice[start_index + 1] = green;
        slice[start_index + 2] = red;
    }

    #[cfg(test)]
    #[inline]
    pub fn set_pixel_v2(&self, (x, y): (usize, usize), bgr: [u8; 3]) {
        if x >= self.width() || y >= self.height() {
            return;
        }
        let start_index = y * self.line_length() + x * self.bytes_per_pixel();

        let slice = self.slice_mut();
        slice[start_index..start_index + 3].clone_from_slice(&bgr);
    }

    pub fn set_pixel(&self, (x, y): (usize, usize), (red, green, blue): (u8, u8, u8)) {
        if x >= self.width() || y >= self.height() {
            return;
        }
        let start_index = y * self.line_length() + x * self.bytes_per_pixel();
        let slice = self.slice_mut();
        debug_assert!(slice.len() > start_index + 2);
        unsafe {
            *slice.get_unchecked_mut(start_index) = blue;
            *slice.get_unchecked_mut(start_index + 1) = green;
            *slice.get_unchecked_mut(start_index + 2) = red;
        }
    }

    pub fn get_size_message(&self) -> &[u8] {
        &self.frame_size_message
    }

    pub fn render(&self) {}
}

macro_rules! bench_set_pixel {
    ($fn_name:ident, $bracket_style:tt) => {
        bench_set_pixel!($fn_name, $fn_name, $bracket_style);
    };
    ($mod_name:ident, $fn_name:ident, $bracket_style:tt) => {
        pub mod $mod_name {
            #[cfg(test)]
            #[bench]
            pub fn bench(b: &mut test::Bencher) {
                let screen = super::Screen::init();
                b.iter(|| {
                    for r in 0..10 {
                        for g in 0..10 {
                            for b in 0..10 {
                                for x in 0..10 {
                                    for y in 0..10 {
                                        bench_set_pixel!(
                                            screen,
                                            impl $fn_name,
                                            $bracket_style,
                                            x,
                                            y,
                                            r,
                                            g,
                                            b
                                        );
                                    }
                                }
                            }
                        }
                    }
                });
            }
        }
    };
    ($screen:expr, impl $fn_name:ident, (), $x:expr, $y:expr, $r:expr, $g:expr, $b:expr) => {
        $screen.$fn_name(($x, $y), ($r, $g, $b))
    };
    ($screen:expr, impl $fn_name:ident, [], $x:expr, $y:expr, $r:expr, $g:expr, $b:expr) => {
        $screen.$fn_name(($x, $y), [$r, $g, $b])
    };
}

bench_set_pixel!(set_pixel_v1, ());
bench_set_pixel!(set_pixel_v2, []);
bench_set_pixel!(set_pixel_v3, set_pixel, ());
