extern crate clap;
extern crate sdl2;
extern crate snap;

use clap::{App, Arg};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::time::Duration;

fn main() {
    let matches = App::new("Pixelflut display")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .help("Width in pixels, default is 800"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true)
                .help("Height in pixels, default is 600"),
        )
        .arg(
            Arg::with_name("fullscreen")
                .short("f")
                .help("Start the application in fullscreen mode"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("Port to bind to, will be random by default"),
        )
        .get_matches();
    let context = sdl2::init().expect("Could not start SDL2");
    let video = context.video().expect("Could not start video subsystem");
    let mut event_pump = context.event_pump().expect("Could not start event pump");

    let width = matches
        .value_of("width")
        .and_then(|w| w.parse().ok())
        .unwrap_or(800);
    let height = matches
        .value_of("height")
        .and_then(|h| h.parse().ok())
        .unwrap_or(600);
    let buffer_size = (width * height * 3) as usize;

    let mut window = video.window("rust-sdl2 demo", width, height);
    window.position_centered();
    if matches.is_present("fullscreen") {
        window.fullscreen();
    }
    let window = window.build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let addr: SocketAddr = format!("0.0.0.0:{}", matches.value_of("port").unwrap_or("0"))
        .parse()
        .unwrap();
    let stream = TcpListener::bind(addr).unwrap();
    println!("Listening on {:?}", stream.local_addr().unwrap());
    stream.set_nonblocking(true).unwrap();
    let mut clients = Vec::new();

    let mut recv_buffer = Vec::new();
    recv_buffer.resize(buffer_size, 0);
    let texture_creator = canvas.texture_creator();

    let mut texture = {
        let mut surface = Surface::new(width, height, PixelFormatEnum::RGB24).unwrap();
        surface
            .fill_rect(Some(Rect::new(0, 0, width, height)), Color::RGB(0, 255, 0))
            .unwrap();
        texture_creator.create_texture_from_surface(surface).unwrap()
    };

    'running: loop {
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        {
            match stream.accept() {
                Ok((c, _)) => {
                    c.set_nonblocking(true).unwrap();
                    clients.push((snap::Reader::new(c), Vec::new()));
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => {
                    panic!("{:?}", e);
                }
            }
        }

        let mut died_clients: Vec<usize> = Vec::new();
        {
            for (index, (ref mut client, ref mut buffer)) in clients.iter_mut().enumerate() {
                match client.read(&mut recv_buffer) {
                    Ok(l) => {
                        buffer.extend(&recv_buffer[..l]);
                        if buffer.len() >= buffer_size {
                            client.get_ref().write(b"ACK\n").unwrap();
                            let mut drained = buffer.drain(..buffer_size).collect::<Vec<_>>();
                            let surface = Surface::from_data(
                                &mut drained,
                                width,
                                height,
                                width * 3,
                                PixelFormatEnum::RGB24,
                            ).unwrap();
                            texture = texture_creator.create_texture_from_surface(surface).unwrap();
                        }
                    }
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
                    Err(e) => {
                        println!("Client died {:?}", e);
                        died_clients.push(index);
                    }
                }
            }
        }
        {
            for i in died_clients.into_iter().rev() {
                clients.remove(i);
            }
        }

        ::std::thread::sleep(Duration::from_millis(10));
    }
}
