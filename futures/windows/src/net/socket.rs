use super::SocketAddress;
use crate::Win32Event;
use executor::{Error, Result};
use std::ffi::{c_int, c_long};
use win32::{
    try_wsa_get_last_error,
    winsock2::{
        bind, closesocket, ioctlsocket, listen, socket, WSAEventSelect, FIONBIO, INVALID_SOCKET,
        SOCKET,
    },
};

/// A Win32 socket
pub(super) struct Socket(SOCKET);

impl Socket {
    /// Creates a new stream socket
    pub(super) fn new(af: c_int, r#type: c_int) -> Result<Self> {
        let socket = unsafe { socket(af, r#type, 0) };
        if socket == INVALID_SOCKET {
            return Err(Error::wsa_get_last_error());
        }

        Ok(Socket(socket))
    }

    /// Binds the socket to `sockaddr`
    pub(super) fn bind(&mut self, sockaddr: SocketAddress) -> Result<()> {
        try_wsa_get_last_error!(bind(self.0, sockaddr.as_ptr(), sockaddr.namelen())).map(|_| ())
    }

    /// Begins accepting connections on this socket
    pub(super) fn listen(&mut self, backlog: c_int) -> Result<()> {
        try_wsa_get_last_error!(listen(self.0, backlog)).map(|_| ())
    }

    /// Sets this socket to be non-blocking
    pub(super) fn set_non_blocking(&mut self) -> Result<()> {
        let mut mode = 1;
        try_wsa_get_last_error!(ioctlsocket(self.0, FIONBIO, &mut mode)).map(|_| ())
    }

    /// Signal `event` when any `network_events` fire
    ///
    /// # Safety
    /// This function is unsafe because it cannot guarantee the lifetimes of the event and socket
    /// with relation to each other. Incorrect usage here won't crash the program but can lead to
    /// unexpected results.
    pub(super) unsafe fn event_select(
        &mut self,
        event: &mut Win32Event,
        network_events: c_long,
    ) -> Result<()> {
        try_wsa_get_last_error!(WSAEventSelect(self.0, event.inner(), network_events)).map(|_| ())
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { closesocket(self.0) };
    }
}
