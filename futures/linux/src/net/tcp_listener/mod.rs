use super::Socket;
use executor::Result;
use linux::sys::socket::{AF_INET, AF_INET6, SOMAXCONN};
use std::net::SocketAddr;

/// A listening socket for TCP connections
pub struct TCPListener {
    socket: Socket,
}

impl TCPListener {
    /// Creates a new [`TCPListener`] bound to `addr`
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let family = match addr {
            SocketAddr::V4(_) => AF_INET,
            SocketAddr::V6(_) => AF_INET6,
        };

        let mut socket = Socket::new(family)?;
        socket.bind(addr)?;
        socket.listen(SOMAXCONN)?;

        Ok(TCPListener { socket })
    }
}
