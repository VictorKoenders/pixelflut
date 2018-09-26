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
    pub fn set_pixel((x, y): (usize, usize), (red, green, blue): (u8, u8, u8)) {
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
    pub fn render(&self) {}
}
