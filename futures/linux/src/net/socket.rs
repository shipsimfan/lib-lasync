use executor::Result;
use linux::{
    netinet::r#in::sockaddr_in,
    sys::socket::{bind, listen, socket, AF_INET},
    try_linux,
    unistd::close,
};
use std::ffi::c_int;

/// A Linux socket
pub(super) struct Socket(c_int);

impl Socket {
    /// Creates a new socket
    pub(super) fn new(r#type: c_int) -> Result<Self> {
        let fd = try_linux!(socket(AF_INET, r#type, 0))?;

        Ok(Socket(fd))
    }

    /// Binds the socket to `addr`
    pub(super) fn bind(&mut self, addr: &sockaddr_in) -> Result<()> {
        try_linux!(bind(
            self.0,
            addr as *const _ as _,
            std::mem::size_of::<sockaddr_in>() as _
        ))
        .map(|_| ())
    }

    /// Begins accepting connections on this socket
    pub(super) fn listen(&mut self, backlog: c_int) -> Result<()> {
        try_linux!(listen(self.0, backlog)).map(|_| ())
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { close(self.0) };
    }
}
