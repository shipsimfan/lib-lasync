use executor::Result;
use linux::{
    netinet::r#in::{in_addr, sockaddr_in},
    sys::socket::{bind, listen, socket, AF_INET, SOCK_STREAM},
    try_linux,
    unistd::close,
};
use std::{ffi::c_int, net::SocketAddr};

/// A listening socket for TCP connections
pub struct TCPListener {
    fd: c_int,
}

impl TCPListener {
    /// Creates a new [`TCPListener`] bound to `addr`
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let serv_addr = sockaddr_in {
            family: AF_INET as _,
            addr: in_addr { addr: 0 },
            port: addr.port().to_be(),
            zero: [0; 8],
        };

        let fd = try_linux!(socket(AF_INET, SOCK_STREAM, 0))?;

        try_linux!(bind(
            fd,
            (&serv_addr as *const sockaddr_in).cast(),
            std::mem::size_of::<sockaddr_in>() as _
        ))?;

        try_linux!(listen(fd, 10))?;

        Ok(TCPListener { fd })
    }
}

impl Drop for TCPListener {
    fn drop(&mut self) {
        unsafe { close(self.fd) };
    }
}
