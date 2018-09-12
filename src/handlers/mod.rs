/// a handler that takes an entire CPU to itself to handle clients.
/// It accepts a receiver, which can be used from the main thread to assign
/// TcpStreams to this handler.
pub mod cpu_handler;

/// a handler that spawns a new thread for each incoming connection
pub mod max_threads;
