use super::{SocketAddress, TCPListener};
use crate::{net::TCPStream, EventRef};
use executor::{platform::EventHandler, EventID, EventManager, Result};
use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};
use uring::io_uring_cqe;

/// A future which yields a new connection from a [`TCPListener`]
pub struct Accept<'a> {
    /// The listening socket to accept from
    listener: &'a TCPListener,

    /// The event ID this is registered under
    event_id: EventRef,

    /// The space to for the incoming clients socket address
    socket_address: SocketAddress,

    /// Has the SQE been submitted?
    sqe_submitted: bool,
}

/// The callback for when a client is accepted
fn accept_callback(_: &mut io_uring_cqe, _: &mut usize) {
    todo!("accept_callback()");
}

impl<'a> Accept<'a> {
    /// Creates a new [`Accept`] future
    pub(super) fn new(listener: &'a TCPListener) -> Result<Self> {
        let event_id = EventRef::register(EventHandler::new(0, accept_callback))?;

        let socket_address = SocketAddress::default(listener.socket_family);

        Ok(Accept {
            listener,
            event_id,
            socket_address,
            sqe_submitted: false,
        })
    }

    /// Projects pinned self into `(self.listener, self.event_id, self.socket_address,
    /// self.sqe_submitted)`
    ///
    /// # SAFTEY
    /// This is the only way to access the contained [`SocketAddress`], do not acces it
    /// directly.
    fn project(
        self: Pin<&mut Self>,
    ) -> (&TCPListener, EventID, Pin<&mut SocketAddress>, &mut bool) {
        let this = unsafe { self.get_unchecked_mut() };

        (
            this.listener,
            *this.event_id,
            Pin::new(&mut this.socket_address),
            &mut this.sqe_submitted,
        )
    }
}

impl<'a> Future for Accept<'a> {
    type Output = Result<(TCPStream, SocketAddr)>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (listener, event_id, socket_address, sqe_submitted) = unsafe { self.project() };

        EventManager::get_local_mut(|manager| {
            // Submit the SQE if one hasn't been submitted yet
            if !*sqe_submitted {
                let sqe = manager.get_sqe(event_id).unwrap();

                todo!("io_uring_prep_accept()");

                sqe.submit().unwrap();
                *sqe_submitted = true;
            }

            todo!("Check if the accept is complete");
        })
    }
}
