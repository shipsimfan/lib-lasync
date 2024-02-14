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
    /// Poll for an event
    fn poll(&self);
}

impl Pollable for () {
    fn poll(&self) {}
}
