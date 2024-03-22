use linux::{
    netinet::r#in::{in6_addr, in_addr, sockaddr_in, sockaddr_in6},
    sys::socket::{sockaddr, AF_INET, AF_INET6},
};
use std::{ffi::c_int, net::SocketAddr};

/// An address associated with a socket
#[derive(Clone)]
pub(super) enum SocketAddress {
    /// IPv4
    V4(sockaddr_in),

    /// IPv6
    V6(sockaddr_in6),
}

impl SocketAddress {
    pub(super) fn default(family: c_int) -> Self {
        match family {
            AF_INET => SocketAddress::V4(sockaddr_in {
                family: AF_INET as _,
                port: 0,
                addr: in_addr { addr: 0 },
                zero: [0; 8],
            }),
            AF_INET6 => SocketAddress::V6(sockaddr_in6 {
                family: AF_INET6 as _,
                port: 0,
                flow_info: 0,
                addr: in6_addr { addr: [0; 16] },
                scope_id: 0,
            }),
            _ => panic!("unknown socket family {}", family),
        }
    }

    /// Gets the family of this socket address
    pub(super) fn family(&self) -> c_int {
        match self {
            SocketAddress::V4(_) => AF_INET,
            SocketAddress::V6(_) => AF_INET6,
        }
    }

    pub(super) fn len(&self) -> usize {
        match self {
            SocketAddress::V4(_) => std::mem::size_of::<sockaddr_in>(),
            SocketAddress::V6(_) => std::mem::size_of::<sockaddr_in6>(),
        }
    }

    /// Gets the pointer to the underlying socket address
    pub(super) fn as_ptr(&self) -> *const sockaddr {
        match self {
            SocketAddress::V4(addr) => (addr as *const sockaddr_in).cast(),
            SocketAddress::V6(addr) => (addr as *const sockaddr_in6).cast(),
        }
    }
}

impl From<SocketAddr> for SocketAddress {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(addr) => SocketAddress::V4(sockaddr_in {
                family: AF_INET as _,
                addr: in_addr {
                    addr: addr.ip().to_bits().to_be(),
                },
                port: addr.port().to_be(),
                zero: [0; 8],
            }),
            SocketAddr::V6(addr) => SocketAddress::V6(sockaddr_in6 {
                family: AF_INET6 as _,
                port: addr.port().to_be(),
                flow_info: addr.flowinfo().to_be(),
                addr: in6_addr {
                    addr: addr.ip().octets(),
                },
                scope_id: addr.scope_id().to_be(),
            }),
        }
    }
}
