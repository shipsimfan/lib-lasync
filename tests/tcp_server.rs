use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

const PORT: u16 = 8192;
const SOCKET_ADDRESS: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, PORT));

#[test]
fn tcp_server() {
    let tcp_listener = lasync::futures::net::TCPListener::bind(SOCKET_ADDRESS).unwrap();

    loop {
    }
}
