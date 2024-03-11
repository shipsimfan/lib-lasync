use crate::{
    net::{socket_address::SocketAddress, TCPListener, TCPStream},
    EventRef,
};
use executor::Result;
use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};

/// A future which yields a new connection from a [`TCPListener`]
pub struct Accept<'a> {
    /// The socket on which to accept a connection
    listener: &'a mut TCPListener,

    /// The event id this is registered under
    #[allow(unused)]
    event: EventRef,
}

impl<'a> Accept<'a> {
    /// Creates a new [`Accept`] future
    pub(super) fn new(listener: &'a mut TCPListener) -> Result<Self> {
        let event = EventRef::register_handle(listener.event_handle())?;

        Ok(Accept { listener, event })
    }
}

impl<'a> Future for Accept<'a> {
    type Output = Result<(TCPStream, SocketAddr)>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut socket_address = SocketAddress::empty();
        match self.listener.socket().accept(&mut socket_address) {
            Ok(None) => Poll::Pending,
            Ok(Some(stream)) => Poll::Ready(Ok((stream, socket_address.into()))),
            Err(error) => Poll::Ready(Err(error)),
        }
    }
}
