use crate::handlers::{cpu_handler, max_threads, Interrupter}; // async_handler,
use crate::screen::Screen;
use rand::{thread_rng, Rng};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[test]
pub fn cpu_bound() {
    do_test(|interrupt, port| {
        const NUM_CPU: usize = 2;
        cpu_handler::main_loop([127, 0, 0, 1].into(), port, NUM_CPU, interrupt)
    });
}

#[test]
pub fn max_threads() {
    do_test(|interrupt, port| max_threads::main_loop([127, 0, 0, 1].into(), port, interrupt));
}

/*#[test]
pub fn r#async() {
    do_test(|interrupt, port| async_handler::main_loop([127, 0, 0, 1].into(), port, 2, interrupt));
}*/

fn do_test<F>(f: F)
where
    F: Fn(&dyn Interrupter, u16) + std::marker::Send + 'static,
{
    Screen::init();
    let mut rng = thread_rng();
    let interrupter = TestInterrupter {
        val: Arc::new(AtomicBool::new(true)),
    };
    let interrupter_clone = interrupter.clone();
    let port = rng.gen_range(1500, 2500);

    println!("Listening on {:?}", port);

    // TODO: Check if port isn't already in use
    std::thread::spawn(move || f(&*interrupter_clone, port));
    std::thread::sleep(std::time::Duration::from_millis(100));

    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("Could not connect to server");
    stream
        .write_all(b"SIZE\n")
        .expect("Could not send SIZE command");
    let mut buffer = [0u8; 1024];
    let length = stream
        .read(&mut buffer[..])
        .expect("Could not read SIZE response");
    let response = std::str::from_utf8(&buffer[..length]).expect("Could not parse SIZE response");
    let mut split = response.split(' ').map(|s| s.trim());
    let _ = split.next();
    let width: usize = split.next().unwrap().parse().unwrap();
    let height: usize = split.next().unwrap().parse().unwrap();
    assert_eq!(split.next(), None);

    println!("Screen size {}x{}", width, height);

    assert_eq!(Screen::get_pixel_at(50, 50), Some(&[0, 0, 0][..]));
    for (command, expected) in &[
        ("FF0000", [0, 0, 255]),
        ("00FF00", [0, 255, 0]),
        ("0000FF", [255, 0, 0]),
    ] {
        let command = format!("PX {} {} {}\n", 50, 50, command);
        stream
            .write_all(command.as_bytes())
            .expect("Could not send PX command");
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(Screen::get_pixel_at(50, 50), Some(&expected[..]));
    }

    println!("Filling entire screen, this might take a while...");

    for x in 0..width {
        for y in 0..height {
            let command = format!("PX {} {} FFFFFF\n", x, y);
            stream
                .write_all(command.as_bytes())
                .expect("Could not send PX command");
        }
    }
    // Give the process some time to handle the data
    std::thread::sleep(std::time::Duration::from_millis(5000));
    println!("Checking if all pixels are white");

    assert!(Screen::all([255, 255, 255]));
    println!("Interrupting test");
    interrupter.val.store(false, Ordering::Relaxed);
    println!("Done!");
}

struct TestInterrupter {
    val: Arc<AtomicBool>,
}

impl Interrupter for TestInterrupter {
    fn is_running(&self) -> bool {
        self.val.load(Ordering::Relaxed)
    }
    fn clone(&self) -> Box<dyn crate::handlers::Interrupter> {
        Box::new(TestInterrupter {
            val: Arc::clone(&self.val),
        })
    }
}
