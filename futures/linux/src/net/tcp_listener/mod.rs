use super::Socket;
use executor::Result;
use linux::{
    netinet::r#in::{in_addr, sockaddr_in},
    sys::socket::{AF_INET, SOCK_STREAM, SOMAXCONN},
};
use std::net::SocketAddr;

/// A listening socket for TCP connections
pub struct TCPListener {
    /// The underlying socket
    socket: Socket,
}

impl TCPListener {
    /// Creates a new [`TCPListener`] bound to `addr`
    pub fn bind(addr: SocketAddr) -> Result<Self> {
        let addr = if let SocketAddr::V4(addr) = addr {
            sockaddr_in {
                family: AF_INET as _,
                port: addr.port().reverse_bits(),
                addr: in_addr {
                    addr: addr.ip().to_bits(),
                },
                zero: [0; 8],
            }
        } else {
            panic!("lasync currently does not support IPv6");
        };

        let mut socket = Socket::new(SOCK_STREAM)?;
        socket.bind(&addr)?;
        socket.listen(SOMAXCONN)?;

        Ok(TCPListener { socket })
    }
}
