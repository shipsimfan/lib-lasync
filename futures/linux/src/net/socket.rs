use executor::Result;
use linux::{
    netinet::r#in::{in6_addr, in_addr, sockaddr_in, sockaddr_in6},
    sys::socket::{bind, listen, sockaddr, socket, AF_INET, AF_INET6, SOCK_STREAM},
    try_linux,
    unistd::close,
};
use std::{ffi::c_int, net::SocketAddr};

/// A Linux socket
pub(super) struct Socket(c_int);

impl Socket {
    /// Creates a new unbound [`Socket`]
    pub(super) fn new() -> Result<Self> {
        try_linux!(socket(AF_INET, SOCK_STREAM, 0)).map(|fd| Socket(fd))
    }

    /// Binds this socket to `addr` (IPv4)
    pub(super) fn bind(&mut self, addr: SocketAddr) -> Result<()> {
        match addr {
            SocketAddr::V4(addr) => {
                let addr = sockaddr_in {
                    family: AF_INET as _,
                    addr: in_addr {
                        addr: addr.ip().to_bits(),
                    },
                    port: addr.port().to_be(),
                    zero: [0; 8],
                };

                self.bind_v4(&addr)
            }
            SocketAddr::V6(addr) => {
                let addr = sockaddr_in6 {
                    family: AF_INET6 as _,
                    port: addr.port().to_be(),
                    flow_info: addr.flowinfo(),
                    addr: in6_addr {
                        addr: addr.ip().octets(),
                    },
                    scope_id: addr.scope_id(),
                };

                self.bind_v6(&addr)
            }
        }
    }

    /// Sets this socket into a listen state, allowing this socket to accept incoming connections
    pub(super) fn listen(&mut self, backlog: c_int) -> Result<()> {
        try_linux!(listen(self.0, backlog)).map(|_| ())
    }

    /// Binds this socket to `addr` (IPv4)
    fn bind_v4(&mut self, addr: *const sockaddr_in) -> Result<()> {
        self.do_bind(addr.cast(), std::mem::size_of::<sockaddr_in>())
    }

    /// Binds this socket to `addr` (IPv6)
    fn bind_v6(&mut self, addr: *const sockaddr_in6) -> Result<()> {
        self.do_bind(addr.cast(), std::mem::size_of::<sockaddr_in6>())
    }

    /// Binds this socket to `addr`
    fn do_bind(&mut self, addr: *const sockaddr, addrlen: usize) -> Result<()> {
        try_linux!(bind(self.0, addr, addrlen as _)).map(|_| ())
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { close(self.0) };
    }
}
