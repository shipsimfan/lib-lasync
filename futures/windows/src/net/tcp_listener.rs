use crate::net::socket_address::SocketAddress;
use executor::{Error, Result};
use std::net::SocketAddr;
use win32::{
    try_wsa_get_last_error,
    winsock2::{bind, closesocket, socket, INVALID_SOCKET, SOCKET, SOCK_STREAM},
};

/// A listening socket for TCP connection
pub struct TCPListener(SOCKET);

impl TCPListener {
    /// Creates a new [`TCPListener`] bound to `addr`
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let sockaddr: SocketAddress = addr.into();

        let socket = unsafe { socket(sockaddr.family() as _, SOCK_STREAM, 0) };
        if socket == INVALID_SOCKET {
            return Err(Error::wsa_get_last_error());
        }

        try_wsa_get_last_error!(unsafe { bind(socket, sockaddr.as_ptr(), sockaddr.namelen()) })?;

        Ok(TCPListener(socket))
    }
}

impl Drop for TCPListener {
    fn drop(&mut self) {
        unsafe { closesocket(self.0) };
    }
}
