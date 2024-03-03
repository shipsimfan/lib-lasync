use executor::{Error, Result};
use std::net::SocketAddr;
use win32::winsock2::{
    closesocket, socket, AF_INET, AF_INET6, INVALID_SOCKET, SOCKET, SOCK_STREAM,
};

/// A listening socket for TCP connection
pub struct TCPListener(SOCKET);

impl TCPListener {
    /// Creates a new [`TCPListener`] bound to `addr`
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let af = match &addr {
            SocketAddr::V4(_) => AF_INET,
            SocketAddr::V6(_) => AF_INET6,
        };

        let socket = unsafe { socket(af, SOCK_STREAM, 0) };
        if socket == INVALID_SOCKET {
            return Err(Error::wsa_get_last_error());
        }

        todo!("bind()");

        Ok(TCPListener(socket))
    }
}

impl Drop for TCPListener {
    fn drop(&mut self) {
        unsafe { closesocket(self.0) };
    }
}
