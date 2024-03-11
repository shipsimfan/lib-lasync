use super::Socket;
use win32::winsock2::SOCKET;

/// A TCP stream between a local and a remote socket
pub struct TCPStream {
    socket: Socket,
}

impl TCPStream {
    /// Creates a [`TCPStream`] from a raw Win32 [`SOCKET`]
    pub(super) unsafe fn from_raw(socket: SOCKET) -> Self {
        TCPStream {
            socket: Socket::from_raw(socket),
        }
    }
}
