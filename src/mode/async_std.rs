use crate::{
    client::ClientState,
    screen::{Screen, ScreenUpdater},
};
use async_std::{
    io::{ReadExt, WriteExt},
    net::{TcpListener, TcpStream},
};

pub fn start(args: crate::Args, screen: impl Screen, updater: Option<impl ScreenUpdater>) {
    if let Some(count) = args.core_count {
        // ASYNC_STD_THREAD_COUNT: The number of threads that the async-std runtime will start.
        // By default, this is one per logical cpu as reported by the num_cpus crate, which may be different than the number of physical cpus.
        // Async-std will panic if this is set to any value other than a positive integer.
        std::env::set_var("ASYNC_STD_THREAD_COUNT", count.to_string());
    }

    let run = move || {
        async_std::task::block_on(async move {
            let listener = TcpListener::bind((args.host.as_str(), args.port))
                .await
                .expect("Could not listen on host");

            println!("Listening on {:?}", listener.local_addr().unwrap());

            while screen.running() {
                let (client, _addr) = listener
                    .accept()
                    .await
                    .expect("Could not accept new client");

                async_std::task::spawn(run_client(client, screen.clone()));
            }
        })
    };

    // if we have an updater, run async_std on a background thread
    if let Some(mut updater) = updater {
        println!("Updater detected, running async_std in background thread");
        std::thread::spawn(run);
        while updater.running() {
            updater.update();
        }
    } else {
        run();
    }
}

async fn run_client(mut client: TcpStream, screen: impl Screen) {
    let mut state = ClientState::default();

    while let Ok(len) = client.read(state.recv_buffer()).await {
        if len == 0 {
            break;
        }
        let response = state.parse_buffer(&screen, len);
        if let Some(response) = response {
            let bytes = response.as_bytes();
            if client.write_all(&bytes).await.is_err() {
                break;
            }
        }
        if !screen.running() {
            let _ = client.write_all(b"\r\nServer shutting down\r\n").await;
            break;
        }
    }
}
