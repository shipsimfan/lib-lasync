use crate::{net::TCPListener, EventRef};
use executor::Result;

/// A future which yields a new connection from a [`TCPListener`]
pub struct Accept<'a> {
    /// The socket on which to accept a connection
    listener: &'a mut TCPListener,

    /// The event id this is registered under
    event: EventRef,
}

impl<'a> Accept<'a> {
    /// Creates a new [`Accept`] future
    pub(super) fn new(listener: &'a mut TCPListener) -> Result<Self> {
        todo!("Accept::new()")
    }
}
