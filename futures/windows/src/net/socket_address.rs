use std::{
    ffi::{c_int, c_short},
    mem::ManuallyDrop,
    net::{SocketAddr, SocketAddrV4, SocketAddrV6},
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
    /// Creates a [`SocketAddress`] initialized to zero
    pub(super) fn empty() -> Self {
        SocketAddress {
            v6: ManuallyDrop::new(sockaddr_in6 {
                family: 0,
                port: 0,
                flowinfo: 0,
                addr: [0; 16],
                scope_id: 0,
            }),
        }
    }

    /// Gets the family this address belongs to
    pub(super) fn family(&self) -> c_short {
        unsafe { self.v4.family }
    }

    /// Gets the length of the address
    pub(super) fn namelen(&self) -> c_int {
        match self.family() as i32 {
            AF_INET => std::mem::size_of::<sockaddr_in>() as _,
            AF_INET6 => std::mem::size_of::<sockaddr_in6>() as _,
            _ => unreachable!(),
        }
    }

    /// Gets a pointer to the underlying [`sockaddr`]
    pub(super) fn as_ptr(&self) -> *const sockaddr {
        (self as *const Self).cast()
    }

    /// Gets a mutable pointer to the underlying [`sockaddr`]
    pub(super) fn as_mut_ptr(&mut self) -> *mut sockaddr {
        (self as *mut Self).cast()
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

impl Into<SocketAddr> for SocketAddress {
    fn into(self) -> SocketAddr {
        match self.family() as i32 {
            AF_INET => {
                SocketAddr::V4(unsafe { SocketAddrV4::new(self.v4.addr.into(), self.v4.port) })
            }
            AF_INET6 => SocketAddr::V6(unsafe {
                SocketAddrV6::new(
                    self.v6.addr.into(),
                    self.v6.port,
                    self.v6.flowinfo,
                    self.v6.scope_id,
                )
            }),
            family => panic!("Cannot convert address family {} to a `SocketAddr`", family),
        }
    }
}
