use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    num::NonZeroUsize,
};

const SOCKET_ADDRESS: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));

const SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };

const DATA: &[u8] = include_bytes!("test_data.txt");

#[test]
fn tcp_server_accept() {
    lasync::run(SIZE, async {
        let tcp_listener = lasync::net::TCPListener::bind(SOCKET_ADDRESS).unwrap();
        let address = tcp_listener.local_addr().unwrap();

        let child = std::thread::spawn(move || tcp_server_accept_client(address));

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

fn tcp_server_accept_client(address: SocketAddr) {
    let stream = std::net::TcpStream::connect(address).unwrap();

    drop(stream);
}

#[test]
fn tcp_server_read() {
    use futures::io::Read;

    lasync::run(SIZE, async {
        let tcp_listener = lasync::net::TCPListener::bind(SOCKET_ADDRESS).unwrap();
        let address = tcp_listener.local_addr().unwrap();

        let child = std::thread::spawn(move || tcp_server_read_client(address));

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

        println!("{}", String::from_utf8_lossy(&buffer));

        child.join().unwrap();
    })
    .unwrap();
}

fn tcp_server_read_client(address: SocketAddr) {
    use std::io::Write;

    let mut stream = std::net::TcpStream::connect(address).unwrap();

    stream.write_all(DATA).unwrap();
}

#[test]
fn tcp_server_write() {
    use futures::io::Write;

    lasync::run(SIZE, async {
        let tcp_listener = lasync::net::TCPListener::bind(SOCKET_ADDRESS).unwrap();
        let address = tcp_listener.local_addr().unwrap();

        let child = std::thread::spawn(move || tcp_server_write_client(address));

        let (mut stream, _) = tcp_listener.accept().await.unwrap();

        let mut buf = DATA;
        while !buf.is_empty() {
            match stream.write(buf).await.unwrap() {
                0 => break,
                n => buf = &buf[n..],
            }
        }

        assert!(buf.is_empty());

        child.join().unwrap();
    })
    .unwrap();
}

fn tcp_server_write_client(address: SocketAddr) {
    use std::io::Read;

    let mut stream = std::net::TcpStream::connect(address).unwrap();

    let mut buffer = [0; DATA.len()];
    stream.read_exact(&mut buffer).unwrap();

    assert_eq!(buffer, DATA);

    println!("{}", String::from_utf8_lossy(&buffer));
}
