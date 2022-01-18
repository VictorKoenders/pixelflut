![](https://raw.githubusercontent.com/VictorKoenders/pixelflut/master/assets/db9c0554-b464-11e9-9eea-abedc0ce7926.gif)

Animation created by [@k_henhey](https://twitter.com/k_henhey)

# Running the server
To run the server, you need to install rustup:

https://rustup.rs/

Make sure `cargo` exists in your PATH. Then you can run the server with the following command:

`cargo run --release --feature <mode>`

On linux, this server will display it's screen on a [framebuffer](https://docs.rs/framebuffer). When running this server on a different operating system, a warning will be printed and no output is shown. The server should still accept connections on the other platforms.

# Modes

The server currently supports 5 modes.

## threadpool
In this mode, the server starts up a thread for each logical core on your machine (as determined by [num_cpus::get](https://docs.rs/num_cpus/latest/num_cpus/fn.get.html)). The number of cores can be optionally configured with a `-c N` flag.

Each incoming connection gets distributed over the different threads. Each thread will loop through the existing connections, see if there is data available, and parse that data.

## max-threads
In this mode, a new thread is spawned for each incoming connection. This results in hundreds or thousands of threads, which are all blocked on the stream for incoming traffic. 

## tokio
In this mode, the server starts up a [tokio](https://tokio.rs/) pool and handles the clients on that. The number of cores can be optionally configured with a `-c N` flag.

## async-std
In this mode, the server starts up an [async-std](https://async.rs/) pool and handles the clients on that. The number of cores can be optionally configured with a `-c N` flag.

## io-uring
In this mode, the server uses Linux's [io_uring](https://en.wikipedia.org/wiki/Io_uring) to handle clients.

This mode can also be combined with `tokio`

This mode requires the `xkbcommon` library to be installed:
- `sudo apt install libxkbcommon-dev`

# Features

Additional features can be enabled. These features can be added to the mode above with commas, e.g. `cargo run --features tokio,windowed`

- `windowed`: Will run pixelflut in a windowed mode instead of on a linux framebuffer. On windows this flag is required.
- `memory-cache`: Will use a memory cache to lookup values very quickly. This uses a large amount of memory (at least 4GB is recommended)

# Recording metrics

## Network traffic

For recording network traffic you can use `tcpdump`.

```bash
sudo tcpdump port 1234 -w tcpdump.capture
```

Then you can read this with
```bash
sudo tcpdump -r tcpdump.capture
```

## Performance

Performance can be recorded and viewed with [flamegraph](https://github.com/flamegraph-rs/flamegraph)

```bash
# For ubuntu
sudo apt install linux-tools-common linux-tools-generic linux-tools-`uname -r`
cargo install flamegraph

# permissions
sudo usermod -aG video $USER # enables framebuffer for non-sudo accounts
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid # enables perf for non-sudo accounts
```

Then run

```bash
flamegraph ./target/release/pixelflut <args>
```

The flamegraph will be expored as `flamegraph.svg`

# Issues and server configuration
Several issues can occur when running the server. These issues are outlined below. When you encounter an issue while running this server, please let us know.

the `max-threads` mode can quickly run your operating system out of usable handles. On linux, you'll most likely want to use [sysctl](http://man7.org/linux/man-pages/man8/sysctl.8.html) for this.

When running the screen on a linux framebuffer, the cursor is not hidden by default. You can run the following command to hide the cursor:
- On the current screen: `echo -n -e '\e[?17;14;224c'`
- On a different TTY screen: `echo -n -e '\e[?17;14;224c' > dev/ttyX`

