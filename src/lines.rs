use bytes::{BufMut, BytesMut};
use futures::try_ready;
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

/// Line based codec
///
/// This decorates a socket and presents a line based read / write interface.
///
/// As a user of `Lines`, we can focus on working at the line level. So, we send
/// and receive values that represent entire lines. The `Lines` codec will
/// handle the encoding and decoding as well as reading from and writing to the
/// socket.
#[derive(Debug)]
pub struct Lines {
    /// The TCP socket.
    socket: TcpStream,
    /// Buffer used when reading from the socket. Data is not returned from this
    /// buffer until an entire line has been read.
    rd: BytesMut,
    /// Buffer used to stage data before writing it to the socket.
    wr: BytesMut,
}

impl Lines {
    /// Create a new `Lines` codec backed by the socket
    pub fn new(socket: TcpStream) -> Self {
        Lines {
            socket,
            rd: BytesMut::with_capacity(1024),
            wr: BytesMut::with_capacity(1024),
        }
    }

    /// Buffer a line.
    ///
    /// This writes the line to an internal buffer. Calls to `poll_flush` will
    /// attempt to flush this buffer to the socket.
    pub fn buffer(&mut self, line: &[u8]) {
        // Silently truncate the line if it would extend our capacity
        if line.len() + self.wr.len() > self.wr.capacity() {
            return;
        }
        // Push the line onto the end of the write buffer.
        //
        // The `put` function is from the `BufMut` trait.
        self.wr.put(line);
    }

    /// Flush the write buffer to the socket
    pub fn poll_flush(&mut self) -> Poll<(), io::Error> {
        // As long as there is buffered data to write, try to write it.
        while !self.wr.is_empty() {
            // Try to write some bytes to the socket
            let n = try_ready!(self.socket.poll_write(&self.wr));

            // As long as the wr is not empty, a successful write should
            // never write 0 bytes.
            assert!(n > 0);

            // This discards the first `n` bytes of the buffer.
            let _ = self.wr.split_to(n);
        }

        Ok(Async::Ready(()))
    }

    /// Read data from the socket.
    ///
    /// This only returns `Ready` when the socket has closed.
    fn fill_read_buf(&mut self) -> Poll<(), io::Error> {
        loop {
            // Read data into the buffer.
            let n = try_ready!(self.socket.read_buf(&mut self.rd));

            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }
}

impl Stream for Lines {
    type Item = BytesMut;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // First, read any new data that might have been received off the socket
        let sock_closed = self.fill_read_buf()?.is_ready();

        // Now, try finding lines
        let mut pos = None;
        for (index, byte) in self.rd.iter().enumerate() {
            if *byte == b'\n' {
                pos = Some((index, 1));
                break;
            }
            if *byte == b'\r' && index + 1 < self.rd.len() && self.rd[index + 1] == b'\n' {
                pos = Some((index, 2));
                break;
            }
        }

        if let Some((pos, length)) = pos {
            // Remove the line from the read buffer and set it to `line`.
            let mut line = self.rd.split_to(pos + length);
            line.split_off(pos);

            // Return the line
            return Ok(Async::Ready(Some(line)));
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}
