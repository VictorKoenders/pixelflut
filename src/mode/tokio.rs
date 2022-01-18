use crate::{
    client::ClientState,
    screen::{Screen, ScreenUpdater},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub fn start(args: crate::Args, screen: impl Screen, updater: Option<impl ScreenUpdater>) {
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    if let Some(cores) = args.core_count {
        builder.max_blocking_threads(cores);
    }
    let rt = builder.enable_all().build().unwrap();

    let run = move || {
        rt.block_on(async move {
            let listener = TcpListener::bind((args.host, args.port))
                .await
                .expect("Could not listen on host");

            println!("Listening on {:?}", listener.local_addr().unwrap());

            while screen.running() {
                let (client, _addr) = listener
                    .accept()
                    .await
                    .expect("Could not accept new client");

                tokio::spawn(run_client(client, screen.clone()));
            }
        })
    };

    // if we have an updater, run tokio on a background thread
    if let Some(mut updater) = updater {
        println!("Updater detected, running tokio in background thread");
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
