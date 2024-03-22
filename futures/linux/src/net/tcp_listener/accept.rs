use super::{SocketAddress, TCPListener};
use crate::{net::TCPStream, EventRef};
use executor::{platform::EventHandler, EventID, EventManager, Result};
use linux::{sys::socket::socklen_t, Error};
use std::{
    ffi::c_int,
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};
use uring::{io_uring_cqe, io_uring_prep_accept, io_uring_prep_cancel64};

/// A future which yields a new connection from a [`TCPListener`]
pub struct Accept<'a> {
    /// The listening socket to accept from
    listener: &'a TCPListener,

    /// The event ID this is registered under
    event_id: EventRef,

    /// The space to for the incoming clients socket address
    socket_address: SocketAddress,

    /// The length of the incoming socket address
    socket_address_len: socklen_t,

    /// Has the SQE been submitted?
    sqe_submitted: bool,
}

/// The bit used to signal completion of the event in the value. 1 means finished. The bit must be
/// in a position greater than 32 to fit the result code from the accept.
const SIGNAL_BIT: usize = 1 << 33;

/// The callback for when a client is accepted
fn accept_callback(cqe: &mut io_uring_cqe, value: &mut usize) {
    *value = (cqe.res as usize) | SIGNAL_BIT;
}

impl<'a> Accept<'a> {
    /// Creates a new [`Accept`] future
    pub(super) fn new(listener: &'a TCPListener) -> Result<Self> {
        let event_id = EventRef::register(EventHandler::new(0, accept_callback))?;

        let socket_address = SocketAddress::default(listener.socket_family);
        let socket_address_len = socket_address.len() as _;

        Ok(Accept {
            listener,
            event_id,
            socket_address,
            socket_address_len,
            sqe_submitted: false,
        })
    }

    /// Projects pinned self into `(self.listener, self.event_id, self.socket_address,
    /// self.socket_address_len, self.sqe_submitted)`
    ///
    /// # SAFTEY
    /// This is the only way to access the contained [`SocketAddress`], do not acces it
    /// directly.
    unsafe fn project(
        self: Pin<&mut Self>,
    ) -> (
        &TCPListener,
        EventID,
        Pin<&mut SocketAddress>,
        Pin<&mut socklen_t>,
        &mut bool,
    ) {
        let this = self.get_unchecked_mut();

        (
            this.listener,
            *this.event_id,
            Pin::new(&mut this.socket_address),
            Pin::new(&mut this.socket_address_len),
            &mut this.sqe_submitted,
        )
    }
}

impl<'a> Future for Accept<'a> {
    type Output = Result<(TCPStream, SocketAddr)>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (listener, event_id, mut socket_address, socket_address_len, sqe_submitted) =
            unsafe { self.project() };

        EventManager::get_local_mut(|manager| {
            // Submit the SQE if one hasn't been submitted yet
            if !*sqe_submitted {
                let sqe = manager.get_sqe(event_id).unwrap();

                unsafe {
                    io_uring_prep_accept(
                        sqe.as_ptr(),
                        listener.socket.fd(),
                        socket_address.as_mut_ptr(),
                        socket_address_len.get_mut(),
                        0,
                    )
                };

                sqe.submit().unwrap();
                *sqe_submitted = true;
            }

            let event = manager.get_event_mut(event_id).unwrap();
            let value = event.get_data().value();
            if value & SIGNAL_BIT == 0 {
                event.set_waker(Some(cx.waker().clone()));
                return Poll::Pending;
            }

            let fd = (value & (u32::MAX as usize)) as c_int;
            if fd < 0 {
                return Poll::Ready(Err(Error::new(-fd)));
            }

            let socket_address: SocketAddr = socket_address.clone().into();
            let tcp_stream = unsafe { TCPStream::from_raw(fd) };
            Poll::Ready(Ok((tcp_stream, socket_address)))
        })
    }
}

impl<'a> Drop for Accept<'a> {
    fn drop(&mut self) {
        if self.sqe_submitted {
            EventManager::get_local_mut(|manager| {
                let sqe = manager.get_sqe(*self.event_id).unwrap();

                unsafe { io_uring_prep_cancel64(sqe.as_ptr(), (*self.event_id).into_u64(), 0) };

                sqe.submit().unwrap();
            })
        }
    }
}
