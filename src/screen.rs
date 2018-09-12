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
    buffer: FrameBuffer,
}
#[cfg(not(target_os = "linux"))]
pub struct Screen {
    _hidden: (),
}

impl Screen {
    pub fn set_pixel((x, y): (usize, usize), (r, g, b): (u8, u8, u8)) {
        unsafe {
            if x >= FRAME_WIDTH || y >= FRAME_HEIGHT {
                return;
            }
            let start_index = (y * FRAME_LINE_LENGTH + x * FRAME_BYTES_PER_PIXEL) as usize;
            FRAME[start_index] = b;
            FRAME[start_index + 1] = g;
            FRAME[start_index + 2] = r;
        }
    }
    pub fn get_screen_size_message() -> &'static [u8] {
        unsafe { &FRAME_SIZE_MESSAGE }
    }

    #[cfg(target_os = "linux")]
    pub fn init() -> Screen {
        let mut buffer = Framebuffer::new("/dev/fb0").expect("Could not open frame buffer");

        let width = buffer.var_screen_info.xres as usize;
        let height = buffer.var_screen_info.yres as usize;
        let line_length = buffer.fix_screen_info.line_length as usize;
        let bytes_per_pixel = buffer.var_screen_info.bits_per_pixel as usize / 8;
        println!("width: {}, height: {}", width, height);
        println!("Listening on {:?}", listener.local_addr());

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
    pub fn render(&self) {
        self.buffer.write_frame(unsafe { &FRAME });
    }

    #[cfg(not(target_os = "linux"))]
    pub fn render(&self) {}
}

/*
pub fn run(handles: &[Handle]) {
    let listener = TcpListener::bind((Ipv4Addr::new(0u8, 0, 0, 0), 1234))
        .expect("Could not bind on port 1234");
    listener
        .set_nonblocking(true)
        .expect("Could not set listener to nonblocking");

    let mut buffer = Framebuffer::new("/dev/fb0").expect("Could not open frame buffer");

    let width = buffer.var_screen_info.xres as usize;
    let height = buffer.var_screen_info.yres as usize;
    let line_length = buffer.fix_screen_info.line_length as usize;
    let bytes_per_pixel = buffer.var_screen_info.bits_per_pixel as usize / 8;
    println!("width: {}, height: {}", width, height);
    println!("Listening on {:?}", listener.local_addr());

    unsafe {
        FRAME = vec![0u8; line_length * height];
        FRAME_WIDTH = width;
        FRAME_HEIGHT = height;
        FRAME_LINE_LENGTH = line_length;
        FRAME_BYTES_PER_PIXEL = bytes_per_pixel;
        FRAME_SIZE_MESSAGE = format!("SIZE {} {}\n", width, height).as_bytes().into();
    }
    let mut target_next_frame_time = time::precise_time_ns();

    loop {
        loop {
            match listener.accept() {
                Ok((client, _)) => {
                    let mut lowest = (0, handles[0].client_count());
                    for i in 1..handles.len() {
                        let count = handles[i].client_count();
                        if count < lowest.1 {
                            lowest = (i, count);
                        }
                    }
                    handles[lowest.0].add_client(client);
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => panic!("Could not accept new client: {:?}", e),
            }
        }
        let current_time = time::precise_time_ns();
        if target_next_frame_time < current_time {
            buffer.write_frame(unsafe { &FRAME });
            target_next_frame_time = current_time + FRAME_DURATION_NS;
        }
    }
}
*/
