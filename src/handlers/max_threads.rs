use client::Client;
use screen::Screen;
use std::io::Read;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::ptr;
use std::thread::{sleep, spawn};
use std::time::Duration;
use Result;

pub fn main_loop(host: IpAddr, port: u16) {
    let screen = Screen::init();

    spawn(move || render_loop(&screen));

    let listener = TcpListener::bind((host, port))
        .unwrap_or_else(|e| panic!("Could not listen on {}:{}: {:?}", host, port, e));
    println!("Listening on {}", listener.local_addr().unwrap());

    loop {
        let (socket, _) = listener
            .accept()
            .expect("Could not accept new TCP connection");
        spawn(|| {
            let _ = run_client(socket);
        });
    }
}

fn render_loop(screen: &Screen) {
    loop {
        screen.render();
        sleep(Duration::from_millis(33));
    }
}

fn run_client(mut socket: TcpStream) -> Result<()> {
    let mut buffer = [0u8; 100];
    let mut start_index = 0;
    loop {
        let len = socket.read(&mut buffer[start_index..])?;
        if len == 0 {
            break;
        }
        let total_len = start_index + len;

        start_index = 0;

        while let Some(index) = buffer.iter().skip(start_index).position(|b| b == &b'\n') {
            let _ = Client.handle_message(&mut socket, &buffer[start_index..start_index + index]);
            start_index += index;
        }
        unsafe {
            ptr::copy(
                &buffer[start_index],
                &mut buffer[0],
                total_len - start_index,
            );
        }
        start_index = total_len - start_index;
    }
    Ok(())
}
