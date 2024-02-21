use std::marker::PhantomData;

/// An object which can be used to poll
///
/// This trait exists for platforms which cannot hold the event manager's [`RefCell`] while polling
/// for events.
///
/// On Windows, this is used because APCs run on the same thread that requested them which allows
/// the APCs to directly call the event's waker.
///
/// On Linux, this is not used.
pub trait Pollable {
    /// An error returned from the poll function
    type Error;

    /// Poll for an event
    fn poll(&self) -> Result<(), Self::Error>;
}

/// A utility type for using when polling can happen in the manager
pub struct NoPoll<E>(PhantomData<E>);

impl<E> NoPoll<E> {
    /// Creates a new [`NoPoll`] instance
    pub const fn new() -> Self {
        NoPoll(PhantomData)
    }
}

impl<E> Pollable for NoPoll<E> {
    type Error = E;

    fn poll(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}
