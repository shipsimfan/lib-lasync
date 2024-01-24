use super::Event;
use linux::unistd::close;
use std::ffi::c_int;

/// A set of [`Event`]s to wait on
pub(crate) struct EventManager {
    /// The events we are waiting on
    events: Vec<Event>,

    /// The `epoll` object to wait with
    epoll: c_int,
}

impl EventManager {
    /// Creates a new [`EventManager`]
    pub(in crate::executor) fn new() -> Self {
        EventManager {
            events: Vec::new(),
            epoll: 0,
        }
    }
}

impl Drop for EventManager {
    fn drop(&mut self) {
        unsafe { close(self.epoll) };
    }
}
