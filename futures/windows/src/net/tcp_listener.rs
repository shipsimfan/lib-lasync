use super::{Socket, SocketAddress};
use crate::Win32Event;
use executor::Result;
use std::net::SocketAddr;
use win32::winsock2::{FD_ACCEPT, SOCK_STREAM, SOMAXCONN};

/// A listening socket for TCP connection
pub struct TCPListener {
    socket: Socket,
    accept_event: Win32Event,
}

impl TCPListener {
    /// Creates a new [`TCPListener`] bound to `addr`
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let sockaddr: SocketAddress = addr.into();

        let mut socket = Socket::new(sockaddr.family() as _, SOCK_STREAM)?;
        socket.bind(sockaddr)?;
        socket.listen(SOMAXCONN)?;
        socket.set_non_blocking()?;

        let mut accept_event = Win32Event::new()?;
        unsafe { socket.event_select(&mut accept_event, FD_ACCEPT as _) }?;

        Ok(TCPListener {
            socket,
            accept_event,
        })
    }
}
