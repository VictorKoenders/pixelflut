use crate::config::Config;
use crate::output::ScreenBuffer;
use crate::parse::Parse;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::{Builder, Runtime};

pub struct Tokio(Runtime);

impl super::Input for Tokio {
    fn spawn<P: Parse + 'static, S: ScreenBuffer + 'static>(
        config: Config,
        parser: P,
        buffer: S,
    ) -> Self {
        let thread_count_target = config.max_threads;
        let thread_counter = Arc::new(AtomicUsize::new(0));

        let on_thread_start = callback!(|thread_counter, thread_count_target| {
            let previous_counter = thread_counter.fetch_add(1, Ordering::Relaxed);
            println!(
                "[Tokio] thread started {}/{}",
                previous_counter + 1,
                thread_count_target
            );
        });

        let on_thread_stop = callback!(|thread_counter, thread_count_target| {
            let previous_counter = thread_counter.fetch_sub(1, Ordering::Relaxed);
            println!(
                "[Tokio] thread stopping {}/{}",
                previous_counter - 1,
                thread_count_target
            );
        });

        let runtime = Builder::new()
            .enable_io()
            .core_threads(config.max_threads)
            .threaded_scheduler()
            .on_thread_start(on_thread_start)
            .on_thread_stop(on_thread_stop)
            .build()
            .expect("Could not build a tokio runtime");

        println!("Attempting to start runtime");
        runtime.spawn(run(config, parser, buffer));
        Self(runtime)
    }
}

async fn run<P: Parse + 'static, S: ScreenBuffer + 'static>(config: Config, parser: P, buffer: S) {
    let mut listener = TcpListener::bind((config.host, config.port))
        .await
        .unwrap_or_else(|e| {
            panic!(
                "Could not bind on {:?} port {}: {:?}",
                config.host, config.port, e
            )
        });
    println!("Listening on {:?} port {}", config.host, config.port);

    loop {
        let (socket, _) = match listener.accept().await {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Could not accept incoming socket connection: {:?}", e);
                continue;
            }
        };
        tokio::spawn(process_socket(socket, parser.clone(), buffer.clone()));
    }
}

async fn process_socket<P: Parse + 'static, S: ScreenBuffer + 'static>(
    mut socket: TcpStream,
    mut parser: P,
    screen: S,
) {
    'outer: loop {
        let result = socket.read(parser.write_buffer()).await;
        let n = match result {
            Err(_) => break 'outer, // Client encountered an error
            Ok(0) => break 'outer,  // Client disconnected
            Ok(n) => n,
        };

        if let Some(val) = parser.parse(n, &screen) {
            match socket.write(val.as_bytes()).await {
                Err(_) => break 'outer,                  // Could not write
                Ok(n) if n != val.len() => break 'outer, // Did not write entire buffer
                _ => {}                                  // Ok
            }
        }
    }
}
