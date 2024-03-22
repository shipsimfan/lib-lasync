use executor::Result;
use linux::{
    sys::socket::{bind, listen, socket, SOCK_STREAM},
    try_linux,
    unistd::close,
};
use std::ffi::c_int;

use super::socket_address::SocketAddress;

/// A Linux socket
pub(super) struct Socket(c_int);

impl Socket {
    /// Creates a new unbound [`Socket`]
    pub(super) fn new(family: c_int) -> Result<Self> {
        try_linux!(socket(family, SOCK_STREAM, 0)).map(|fd| Socket(fd))
    }

    /// Binds this socket to `addr` (IPv4)
    pub(super) fn bind(&mut self, address: &SocketAddress) -> Result<()> {
        try_linux!(bind(self.0, address.as_ptr(), address.len() as _)).map(|_| ())
    }

    /// Sets this socket into a listen state, allowing this socket to accept incoming connections
    pub(super) fn listen(&mut self, backlog: c_int) -> Result<()> {
        try_linux!(listen(self.0, backlog)).map(|_| ())
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { close(self.0) };
    }
}
