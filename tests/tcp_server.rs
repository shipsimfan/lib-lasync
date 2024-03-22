use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    num::NonZeroUsize,
    time::Duration,
};

use futures::io::Read;

const PORT: u16 = 8192;
const SOCKET_ADDRESS: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT));

const SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };

const DATA: &[u8] = b"The quick brown fox jumps over the lazy dog";

#[test]
fn tcp_server_accept() {
    lasync::executor::run(SIZE, async {
        let tcp_listener = lasync::futures::net::TCPListener::bind(SOCKET_ADDRESS).unwrap();

        let child = std::thread::spawn(tcp_server_accept_client);

        let (stream, address) = tcp_listener.accept().await.unwrap();

        println!("Connection from {}", address);

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

#[test]
fn tcp_server_read() {
    lasync::executor::run(SIZE, async {
        let tcp_listener = lasync::futures::net::TCPListener::bind(SOCKET_ADDRESS).unwrap();

        let child = std::thread::spawn(tcp_server_read_client);

        let (mut stream, _) = tcp_listener.accept().await.unwrap();

        let mut buffer = [0; DATA.len()];
        let mut buf = &mut buffer as &mut [u8];
        while !buf.is_empty() {
            match stream.read(buf).await.unwrap() {
                0 => break,
                n => buf = &mut buf[n..],
            }
        }

        assert!(buf.is_empty());
        assert_eq!(buffer, DATA);

        println!("{}", String::from_utf8_lossy(DATA));

        child.join().unwrap();
    })
    .unwrap();
}

fn tcp_server_read_client() {
    let mut stream = std::net::TcpStream::connect(SOCKET_ADDRESS).unwrap();

    stream.write_all(DATA).unwrap();
}
