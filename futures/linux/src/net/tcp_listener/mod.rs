use super::{Socket, SocketAddress};
use executor::Result;
use linux::sys::socket::SOMAXCONN;
use std::{net::SocketAddr, pin::Pin};

/// A listening socket for TCP connections
pub struct TCPListener {
    socket: Socket,
    socket_address: SocketAddress,

    accept_socket_address: Pin<Box<SocketAddress>>,
}

impl TCPListener {
    /// Creates a new [`TCPListener`] bound to `addr`
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let socket_address: SocketAddress = addr.into();

        let mut socket = Socket::new(socket_address.family())?;
        socket.bind(&socket_address)?;
        socket.listen(SOMAXCONN)?;

        let accept_socket_address = Box::pin(SocketAddress::default(socket_address.family()));

        Ok(TCPListener {
            socket,
            socket_address,

            accept_socket_address,
        })
    }
}
