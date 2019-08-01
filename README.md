![](https://raw.githubusercontent.com/VictorKoenders/pixelflut/master/assets/db9c0554-b464-11e9-9eea-abedc0ce7926.gif)

Animation created by [@k_henhey](https://twitter.com/k_henhey)

## Running the server
To run the server, you need to install rustup:

https://rustup.rs/

Make sure `cargo` exists in your PATH. Then you can run the server with the following command:

`cargo run --release -- <mode>`

On linux, this server will display it's screen on a [framebuffer](https://docs.rs/framebuffer). When running this server on a different operating system, a warning will be printed and no output is shown. The server should still accept connections on the other platforms.

## Modes

The server currently supports 3 modes. They are listed below, ranging from the slowest to the fastest mode.

### cpu_bound
In this mode, the server starts up a thread for each logical core on your machine (as determined by [num_cpus::get](https://docs.rs/num_cpus/latest/num_cpus/fn.get.html)). The number of cores can be optionally configured with a `-c N` flag.

Each incoming connection gets distributed over the different threads. Each thread will loop through the existing connections, see if there is data available, and parse that data.

### max_threads
In this mode, a new thread is spawned for each incoming connection. This results in hundreds or thousands of threads, which are all blocked on the stream for incoming traffic. 

### async
In this mode, the server starts up a thread for each logical core on your machine (as determined by [num_cpus::get](https://docs.rs/num_cpus/latest/num_cpus/fn.get.html)). The number of cores can be optionally configured with a `-c N` flag.

Each incoming connection gets distributed over the different threads. Each thread has it's own async loop, new connections get added to a queue and the OS notifies us of data being available.

This uses [mio](https://docs.rs/mio) as an abstraction layer over the different OS implementations (epoll, IOCP and kqueue).

## Issues and server configuration
Several issues can occur when running the server. These issues are outlined below. When you encounter an issue while running this server, please let us know.

the `max_threads` mode can quickly run your operating system out of usable handles. On linux, you'll most likely want to use [sysctl](http://man7.org/linux/man-pages/man8/sysctl.8.html) for this.

When running the screen on a linux framebuffer, the cursor is not hidden by default. You can run the following command to hide the cursor:
- On the current screen: `echo -n -e '\e[?17;14;224c'`
- On a different TTY screen: `echo -n -e '\e[?17;14;224c' > dev/ttyX`
