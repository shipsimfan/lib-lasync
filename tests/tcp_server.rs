use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    num::NonZeroUsize,
};

const PORT: u16 = 8192;
const SOCKET_ADDRESS: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT));

const SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };

#[test]
fn tcp_server_accept() {
    lasync::executor::run(SIZE, async {
        let tcp_listener = lasync::futures::net::TCPListener::bind(SOCKET_ADDRESS).unwrap();

        let child = std::thread::spawn(tcp_server_accept_client);

        let (stream, address) = tcp_listener.accept().unwrap().await.unwrap();

        assert!(address.is_ipv4());
        let addr = match address {
            SocketAddr::V4(addr) => addr,
            _ => unreachable!(),
        };

        assert_eq!(*addr.ip(), Ipv4Addr::new(127, 0, 0, 1));
        drop(stream);

        child.join().unwrap();
    })
    .unwrap();
}

fn tcp_server_accept_client() {
    let stream = std::net::TcpStream::connect(SOCKET_ADDRESS).unwrap();

    drop(stream);
}
