#[cfg(target_os = "linux")]
use framebuffer::Framebuffer;

static mut FRAME: Vec<u8> = Vec::new();
static mut FRAME_WIDTH: usize = 0;
static mut FRAME_HEIGHT: usize = 0;
static mut FRAME_SIZE_MESSAGE: Vec<u8> = Vec::new();
static mut FRAME_LINE_LENGTH: usize = 0;
static mut FRAME_BYTES_PER_PIXEL: usize = 0;

#[cfg(target_os = "linux")]
pub struct Screen {
    buffer: Framebuffer,
}

#[cfg(not(target_os = "linux"))]
pub struct Screen {
    _hidden: (),
}

impl Screen {
    #[cfg(test)]
    pub fn get_pixel_at(x: usize, y: usize) -> &'static [u8] {
        unsafe {
            if x >= FRAME_WIDTH || y >= FRAME_HEIGHT {
                panic!("Out of boundaries");
            }
            let start_index = (y * FRAME_LINE_LENGTH + x * FRAME_BYTES_PER_PIXEL) as usize;
            &FRAME[start_index..start_index + 3]
        }
    }

    #[cfg(test)]
    pub fn all(slice: [u8; 3]) -> bool {
        unsafe {
            for i in 0..FRAME.len() / 3 {
                let i = i * 3;
                if FRAME[i..i + 3] != slice[..] {
                    println!(
                        "position {} does not match: {:?} (expected {:?})",
                        i,
                        &FRAME[i..i + 3],
                        &slice[..]
                    );
                    return false;
                }
            }
        }
        true
    }

    #[cfg(test)]
    pub fn set_pixel_v1((x, y): (usize, usize), (red, green, blue): (u8, u8, u8)) {
        unsafe {
            if x >= FRAME_WIDTH || y >= FRAME_HEIGHT {
                return;
            }
            let start_index = (y * FRAME_LINE_LENGTH + x * FRAME_BYTES_PER_PIXEL) as usize;
            FRAME[start_index] = blue;
            FRAME[start_index + 1] = green;
            FRAME[start_index + 2] = red;
        }
    }

    #[cfg(test)]
    #[inline]
    pub fn set_pixel_v2((x, y): (usize, usize), bgr: [u8; 3]) {
        unsafe {
            if x >= FRAME_WIDTH || y >= FRAME_HEIGHT {
                return;
            }
            let start_index = (y * FRAME_LINE_LENGTH + x * FRAME_BYTES_PER_PIXEL) as usize;
            FRAME[start_index..start_index + 3].clone_from_slice(&bgr);
        }
    }

    pub fn set_pixel((x, y): (usize, usize), (red, green, blue): (u8, u8, u8)) {
        unsafe {
            if x >= FRAME_WIDTH || y >= FRAME_HEIGHT {
                return;
            }
            let start_index = y * FRAME_LINE_LENGTH as usize + x * FRAME_BYTES_PER_PIXEL as usize;
            *FRAME.get_unchecked_mut(start_index) = blue;
            *FRAME.get_unchecked_mut(start_index + 1) = green;
            *FRAME.get_unchecked_mut(start_index + 2) = red;
        }
    }

    pub fn get_screen_size_message() -> &'static [u8] {
        unsafe { &FRAME_SIZE_MESSAGE }
    }

    #[cfg(target_os = "linux")]
    pub fn init() -> Screen {
        let buffer = Framebuffer::new("/dev/fb0").expect("Could not open frame buffer");

        let width = buffer.var_screen_info.xres as usize;
        let height = buffer.var_screen_info.yres as usize;
        let line_length = buffer.fix_screen_info.line_length as usize;
        let bytes_per_pixel = buffer.var_screen_info.bits_per_pixel as usize / 8;
        println!("width: {}, height: {}", width, height);

        unsafe {
            FRAME = vec![0u8; line_length * height];
            FRAME_WIDTH = width;
            FRAME_HEIGHT = height;
            FRAME_LINE_LENGTH = line_length;
            FRAME_BYTES_PER_PIXEL = bytes_per_pixel;
            FRAME_SIZE_MESSAGE = format!("SIZE {} {}\n", width, height).as_bytes().into();
        }
        Screen { buffer }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn init() -> Screen {
        println!("[WARNING] Framebuffer not available on your platform");
        println!("[WARNING] PixelFlut will not render anything");
        Screen { _hidden: () }
    }

    #[cfg(target_os = "linux")]
    pub fn render(&mut self) {
        self.buffer.write_frame(unsafe { &FRAME });
    }

    #[cfg(not(target_os = "linux"))]
    pub fn render(&mut self) {}
}

macro_rules! bench_set_pixel {
    ($fn_name:ident, $bracket_style:tt) => {
        bench_set_pixel!($fn_name, $fn_name, $bracket_style);
    };
    ($mod_name:ident, $fn_name:ident, $bracket_style:tt) => {
        pub mod $mod_name {
            #[bench]
            pub fn bench(b: &mut test::Bencher) {
                let mut _screen = super::Screen::init();
                b.iter(|| {
                    for r in 0..10 {
                        for g in 0..10 {
                            for b in 0..10 {
                                for x in 0..10 {
                                    for y in 0..10 {
                                        bench_set_pixel!(
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
    (impl $fn_name:ident, (), $x:expr, $y:expr, $r:expr, $g:expr, $b:expr) => {
        super::Screen::$fn_name(($x, $y), ($r, $g, $b))
    };
    (impl $fn_name:ident, [], $x:expr, $y:expr, $r:expr, $g:expr, $b:expr) => {
        super::Screen::$fn_name(($x, $y), [$r, $g, $b])
    };
}

bench_set_pixel!(set_pixel_v1, ());
bench_set_pixel!(set_pixel_v2, []);
bench_set_pixel!(set_pixel_v3, set_pixel, ());
