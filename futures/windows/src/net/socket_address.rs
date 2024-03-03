use std::{
    ffi::{c_int, c_short},
    mem::ManuallyDrop,
    net::SocketAddr,
};
use win32::winsock2::{sockaddr, sockaddr_in, sockaddr_in6, AF_INET, AF_INET6};

// rustdoc imports
#[allow(unused_imports)]
use crate::net::TCPListener;

/// A socket address to be used by [`TCPListener`] and [`TCPStream`]
pub(super) union SocketAddress {
    /// An IPv4 address
    v4: ManuallyDrop<sockaddr_in>,

    /// An IPv6 address
    v6: ManuallyDrop<sockaddr_in6>,
}

impl SocketAddress {
    pub(super) fn family(&self) -> c_short {
        unsafe { self.v4.family }
    }

    pub(super) fn namelen(&self) -> c_int {
        match self.family() as i32 {
            AF_INET => std::mem::size_of::<sockaddr_in>() as _,
            AF_INET6 => std::mem::size_of::<sockaddr_in6>() as _,
            _ => unreachable!(),
        }
    }

    pub(super) fn as_ptr(&self) -> *const sockaddr {
        (self as *const Self).cast()
    }
}

impl From<SocketAddr> for SocketAddress {
    fn from(value: SocketAddr) -> Self {
        match value {
            SocketAddr::V4(address) => SocketAddress {
                v4: ManuallyDrop::new(sockaddr_in {
                    family: AF_INET as _,
                    port: address.port(),
                    addr: u32::from_be_bytes(address.ip().octets()),
                    zero: [0; 8],
                }),
            },
            SocketAddr::V6(address) => SocketAddress {
                v6: ManuallyDrop::new(sockaddr_in6 {
                    family: AF_INET6 as _,
                    port: address.port(),
                    flowinfo: address.flowinfo(),
                    addr: address.ip().octets(),
                    scope_id: address.scope_id(),
                }),
            },
        }
    }
}
