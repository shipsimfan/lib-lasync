use super::{Socket, SocketAddress};
use executor::Result;
use linux::sys::socket::SOMAXCONN;
use std::net::SocketAddr;

mod accept;

pub use accept::Accept;

/// A listening socket for TCP connections
pub struct TCPListener(Socket);

impl TCPListener {
    /// Creates a new [`TCPListener`] bound to `addr`
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let socket_address: SocketAddress = addr.into();

        let mut socket = Socket::new(socket_address.family())?;
        socket.bind(&socket_address)?;
        socket.listen(SOMAXCONN)?;

        Ok(TCPListener(socket))
    }

    /// Gets the local address of this socket
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr().map(|address| address.into())
    }

    /// Returns a future which yields when a new client connects to this socket
    pub fn accept(&self) -> Accept {
        Accept::new(self)
    }
}

unsafe impl Send for TCPListener {}
unsafe impl Sync for TCPListener {}
