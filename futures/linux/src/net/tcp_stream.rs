use super::Socket;

/// A TCP stream between a local and a remote socket
pub struct TCPStream {
    /// The underlying socket
    socket: Socket,
}
