use super::{Socket, SocketAddress};
use executor::Result;
use linux::sys::socket::SOMAXCONN;
use std::{ffi::c_int, net::SocketAddr};

mod accept;

pub use accept::Accept;

/// A listening socket for TCP connections
pub struct TCPListener {
    /// The underlying socket
    socket: Socket,

    /// The family this socket was created with
    socket_family: c_int,
}

impl TCPListener {
    /// Creates a new [`TCPListener`] bound to `addr`
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let socket_address: SocketAddress = addr.into();

        let mut socket = Socket::new(socket_address.family())?;
        socket.bind(&socket_address)?;
        socket.listen(SOMAXCONN)?;

        Ok(TCPListener {
            socket,
            socket_family: socket_address.family(),
        })
    }

    /// Returns a future which yields when a new client connects to this socket
    pub fn accept(&self) -> Accept {
        Accept::new(self)
    }
}
