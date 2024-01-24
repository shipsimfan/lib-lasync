use std::{ffi::c_int, task::Waker};

/// An I/O event
pub(super) struct Event {
    /// The file descriptor being waited on by this event
    file_descriptor: c_int,

    /// The waker to call if this event is triggered
    waker: Waker,
}
