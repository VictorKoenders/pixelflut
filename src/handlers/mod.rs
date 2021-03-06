/// a handler that takes an entire CPU to itself to handle clients.
/// It accepts a receiver, which can be used from the main thread to assign
/// TcpStreams to this handler.
pub mod cpu_handler;

// /// a handler that deals with incoming requests in an async way.
// pub mod async_handler;

/// a handler that spawns a new thread for each incoming connection
pub mod max_threads;

pub trait Interrupter: std::marker::Send {
    fn is_running(&self) -> bool;
    fn clone(&self) -> Box<dyn Interrupter>;
}

pub struct RunIndefinitely;

impl Interrupter for RunIndefinitely {
    fn is_running(&self) -> bool {
        true
    }
    fn clone(&self) -> Box<dyn Interrupter> {
        Box::new(RunIndefinitely)
    }
}
