use super::Socket;
use crate::{
    fd::FDWrite,
    io::{Read, Write},
    AsFD, FDRead,
};
use std::ffi::c_int;

/// A TCP stream between a local and a remote socket
pub struct TCPStream {
    /// The underlying socket
    socket: Socket,
}

impl TCPStream {
    /// Creates a [`TCPStream`] directly from `fd`
    pub(super) unsafe fn from_raw(fd: c_int) -> Self {
        let socket = Socket::from_raw(fd);

        TCPStream { socket }
    }
}

impl AsFD for TCPStream {
    unsafe fn fd(&self) -> c_int {
        self.socket.fd()
    }
}

impl Read for TCPStream {
    fn read<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> impl std::future::Future<Output = linux::Result<usize>> + 'a {
        FDRead::new(self, buf)
    }
}

impl Write for TCPStream {
    fn write<'a>(
        &'a mut self,
        buf: &'a [u8],
    ) -> impl std::future::Future<Output = linux::Result<usize>> + 'a {
        FDWrite::new(self, buf)
    }
}
