use super::socket_address::SocketAddress;
use executor::{
    platform::linux::{
        netinet::{r#in::IPPROTO_TCP, tcp::TCP_NODELAY},
        sys::socket::{
            bind, getpeername, getsockname, getsockopt, listen, setsockopt, socket, socklen_t,
            SOCK_STREAM, SOL_SOCKET, SO_REUSEADDR,
        },
        try_linux,
        unistd::close,
    },
    Result,
};
use std::ffi::c_int;

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

    /// Gets the family this was created for
    pub(super) fn family(&self) -> c_int {
        self.family
    }

    /// Gets the locally bound address
    pub(super) fn local_addr(&self) -> Result<SocketAddress> {
        let mut address = SocketAddress::default(self.family);
        let mut len = address.len() as socklen_t;

        try_linux!(getsockname(self.fd, address.as_mut_ptr(), &mut len)).map(|_| address)
    }

    /// Gets the remote address of the peer
    pub(super) fn peer_addr(&self) -> Result<SocketAddress> {
        let mut address = SocketAddress::default(self.family);
        let mut len = address.len() as socklen_t;

        try_linux!(getpeername(self.fd, address.as_mut_ptr(), &mut len)).map(|_| address)
    }

    /// Gets if Nagle's algorithm is disabled on this socket
    pub(super) fn nodelay(&self) -> Result<bool> {
        let mut flag: c_int = 0;
        try_linux!(getsockopt(
            self.fd,
            IPPROTO_TCP,
            TCP_NODELAY,
            &mut flag as *mut c_int as _,
            std::mem::size_of::<c_int>() as _
        ))
        .map(|_| flag == 1)
    }

    /// Binds this socket to `addr` (IPv4)
    pub(super) fn bind(&mut self, address: &SocketAddress) -> Result<()> {
        try_linux!(bind(self.fd, address.as_ptr(), address.len() as _)).map(|_| ())
    }

    /// Sets this socket into a listen state, allowing this socket to accept incoming connections
    pub(super) fn listen(&mut self, backlog: c_int) -> Result<()> {
        try_linux!(listen(self.fd, backlog)).map(|_| ())
    }

    /// Sets if this socket will use Nagle's algorithm when sending data
    pub(super) fn set_nodelay(&mut self, nodelay: bool) -> Result<()> {
        let flag = nodelay as c_int;
        try_linux!(setsockopt(
            self.fd,
            IPPROTO_TCP,
            TCP_NODELAY,
            &flag as *const c_int as _,
            std::mem::size_of::<c_int>() as _
        ))
        .map(|_| ())
    }

    /// Sets if the address this socket is bound to can be reused
    pub(super) fn set_reuse_addr(&mut self, reuse_addr: bool) -> Result<()> {
        let flag = reuse_addr as c_int;
        try_linux!(setsockopt(
            self.fd,
            SOL_SOCKET,
            SO_REUSEADDR,
            &flag as *const c_int as _,
            std::mem::size_of::<c_int>() as _
        ))
        .map(|_| ())
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
