use std::{ffi::c_int, task::Waker};

/// An I/O event
pub(super) struct Event {
    /// The file descriptor being waited on by this event
    file_descriptor: c_int,

    /// The waker to call if this event is triggered
    waker: Waker,
}

impl Event {
    /// Creates a new [`Event`]
    pub(super) fn new(file_descriptor: c_int, waker: Waker) -> Self {
        Event {
            file_descriptor,
            waker,
        }
    }

    /// Gets the file descriptor for an event
    pub(super) fn file_descriptor(&self) -> c_int {
        self.file_descriptor
    }

    /// Sets the waker to be called for this event
    pub(super) fn set_waker(&mut self, waker: Waker) {
        self.waker = waker;
    }
}
