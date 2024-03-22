use std::ffi::c_int;

use super::Socket;

/// A TCP stream between a local and a remote socket
pub struct TCPStream {
    /// The underlying socket
    socket: Socket,
}

impl TCPStream {
    /// Creates a [`TCPStream`] directly from `fd`
    pub(super) unsafe fn from_raw(fd: c_int) -> Self {
        let socket = Socket::from_raw(fd);

        TCPStream { socket }
    }
}
