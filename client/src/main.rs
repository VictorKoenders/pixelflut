extern crate num_cpus;
extern crate image;

use std::net::{TcpStream, SocketAddr};
use std::thread::spawn;
use std::io::{Write, BufRead};
use std::time::Duration;
use image::RgbImage;

const TIMEOUT: Duration = Duration::from_secs(5);

fn run(addr: SocketAddr, block: (u32, u32, u32, u32), screen_size: (u32, u32), image: RgbImage){
    let mut commands = Vec::new();
    for x in 0..block.2 {
        for y in 0..block.3 {
            let nearest_pixel = (
                image.width() * (x + block.0) / screen_size.0,
                image.height() * (y + block.1) / screen_size.1,
            );
            let pixel = {
                let pixel = image.get_pixel(nearest_pixel.0, nearest_pixel.1);
                format!("{:02x}{:02x}{:02x}", pixel.data[0], pixel.data[1], pixel.data[2])
            };
            let command = format!("PX {} {} {}\n", block.0 + x, block.1 + y, pixel);
            commands.extend(command.as_bytes());
        }
    }
    let mut stream = TcpStream::connect(addr).unwrap();
    stream.set_write_timeout(Some(TIMEOUT.clone())).unwrap();
    loop {
        stream.write(&commands).unwrap();
    }
}
fn get_dimensions(addr: SocketAddr) -> (u32, u32) {
    let mut stream = TcpStream::connect_timeout(&addr, TIMEOUT.clone()).unwrap();
    stream.set_read_timeout(Some(TIMEOUT.clone())).unwrap();
    stream.set_write_timeout(Some(TIMEOUT.clone())).unwrap();
    stream.write(b"SIZE\n").unwrap();
    let mut reader = std::io::BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    println!("{:?}", line);
    let mut split = line.trim().split(' ');
    split.next().unwrap();
    (split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())
}

fn main() {
    let addr: SocketAddr = ([127u8, 0, 0, 1], 1234).into();
    // let addr: SocketAddr = ([145u8, 116u8, 217u8, 195u8], 1234).into();
    let dimensions = get_dimensions(addr.clone());
    println!("Dimensions are {:?}", dimensions);
    let num_cpus = num_cpus::get() as u32;
    let column_size = dimensions.0 / num_cpus;

    let image = image::open("TrollFace.jpg").unwrap();
    let image = image.to_rgb();

    for i in 0..num_cpus - 1 {
        let addr = addr.clone();
        let image = image.clone();
        spawn(move ||{
            run(addr, (column_size * i, 0, column_size, dimensions.1), dimensions, image);
        });
    }
    run(addr, (dimensions.0 - column_size, 0, column_size, dimensions.1), dimensions, image);
}

