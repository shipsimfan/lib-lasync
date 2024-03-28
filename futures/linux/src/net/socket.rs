use executor::Result;
use linux::{
    sys::socket::{bind, getsockname, listen, socket, SOCK_STREAM},
    try_linux,
    unistd::close,
};
use std::ffi::c_int;

use super::socket_address::SocketAddress;

/// A Linux socket
pub(super) struct Socket {
    /// The underlying socket file descriptor
    fd: c_int,

    /// The family this socket was created for
    family: c_int,
}

impl Socket {
    /// Creates a new unbound [`Socket`]
    pub(super) fn new(family: c_int) -> Result<Self> {
        try_linux!(socket(family, SOCK_STREAM, 0)).map(|fd| Socket { fd, family })
    }

    /// Creates a [`Socket`] from `fd`
    pub(super) unsafe fn from_raw(fd: c_int, family: c_int) -> Self {
        Socket { fd, family }
    }

    pub(super) fn family(&self) -> c_int {
        self.family
    }

    pub(super) fn local_addr(&self) -> Result<SocketAddress> {
        let mut address = SocketAddress::default(self.family);

        try_linux!(getsockname(
            self.fd,
            address.as_mut_ptr(),
            address.len() as _
        ))
        .map(|_| address)
    }

    /// Binds this socket to `addr` (IPv4)
    pub(super) fn bind(&mut self, address: &SocketAddress) -> Result<()> {
        try_linux!(bind(self.fd, address.as_ptr(), address.len() as _)).map(|_| ())
    }

    /// Sets this socket into a listen state, allowing this socket to accept incoming connections
    pub(super) fn listen(&mut self, backlog: c_int) -> Result<()> {
        try_linux!(listen(self.fd, backlog)).map(|_| ())
    }

    /// Gets the underlying file descriptor
    ///
    /// # SAFETY
    /// It is up to the caller to correctly use this file descriptor
    pub(super) unsafe fn fd(&self) -> c_int {
        self.fd
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { close(self.fd) };
    }
}
