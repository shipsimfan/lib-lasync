use super::Socket;
use executor::Result;
use linux::sys::socket::SOMAXCONN;
use std::net::SocketAddr;

/// A listening socket for TCP connections
pub struct TCPListener {
    socket: Socket,
}

impl TCPListener {
    /// Creates a new [`TCPListener`] bound to `addr`
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let mut socket = Socket::new()?;
        socket.bind(addr)?;
        socket.listen(SOMAXCONN)?;

        Ok(TCPListener { socket })
    }
}
