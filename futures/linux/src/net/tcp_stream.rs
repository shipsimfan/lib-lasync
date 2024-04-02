use super::Socket;
use crate::{
    fd::FDWrite,
    io::{Read, Write},
    AsFD, FDRead,
};
use executor::Result;
use std::{ffi::c_int, net::SocketAddr};

/// A TCP stream between a local and a remote socket
pub struct TCPStream(Socket);

impl TCPStream {
    /// Creates a [`TCPStream`] directly from `fd`
    pub(super) unsafe fn from_raw(fd: c_int, family: c_int) -> Self {
        let socket = Socket::from_raw(fd, family);

        TCPStream(socket)
    }

    /// Gets the locally bound address
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.0.local_addr().map(|addr| addr.into())
    }

    /// Gets the remote address of the peer
    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.0.peer_addr().map(|addr| addr.into())
    }
}

impl AsFD for TCPStream {
    unsafe fn fd(&self) -> c_int {
        self.0.fd()
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

unsafe impl Send for TCPStream {}
unsafe impl Sync for TCPStream {}
