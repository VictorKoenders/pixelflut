use framebuffer::Framebuffer;
use handlers::client::Handle;
use std::io::ErrorKind;
use std::net::{Ipv4Addr, TcpListener};
use std::thread::sleep;
use std::time::Duration;
use time;

const FRAME_DURATION_NS: u64 = 1_000_000_000 / 60;

static mut FRAME: Vec<u8> = Vec::new();
static mut FRAME_SIZE_MESSAGE: Vec<u8> = Vec::new();
static mut FRAME_LINE_LENGTH: usize = 0;
static mut FRAME_BYTES_PER_PIXEL: usize = 0;

pub fn set_pixel((x, y): (usize, usize), (r, g, b): (u8, u8, u8)) {
    unsafe {
        let start_index = (y * FRAME_LINE_LENGTH + x * FRAME_BYTES_PER_PIXEL) as usize;
        FRAME[start_index] = b;
        FRAME[start_index + 1] = g;
        FRAME[start_index + 2] = r;
    }
}

pub fn get_screen_size() -> &'static [u8] {
    unsafe { &FRAME_SIZE_MESSAGE }
}

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
        FRAME_LINE_LENGTH = line_length;
        FRAME_BYTES_PER_PIXEL = bytes_per_pixel;
        FRAME_SIZE_MESSAGE = format!("SIZE {} {}\n", width, height).as_bytes().into();
    }

    loop {
        let target_next_frame_time = time::precise_time_ns() + FRAME_DURATION_NS;
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
        buffer.write_frame(unsafe { &FRAME });

        let current_time = time::precise_time_ns();
        if current_time < target_next_frame_time {
            sleep(Duration::from_nanos(target_next_frame_time - current_time));
        }
    }
}
