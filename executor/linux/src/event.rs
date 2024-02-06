use std::{ffi::c_int, task::Waker};

mod id;

pub use id::EventID;

/// A queued event
pub(super) struct Event {
    /// The file descriptor that was inserted into epoll
    file_descriptor: Option<c_int>,

    /// The [`Waker`] used to enqueue the task
    waker: Option<Waker>,
}

impl Event {
    /// Creates a new [`Event`]
    pub(super) fn new() -> Self {
        Event {
            file_descriptor: None,
            waker: None,
        }
    }

    /// Gets the file descriptor associated with this event
    pub(super) fn fd(&self) -> Option<c_int> {
        self.file_descriptor
    }

    pub(super) fn wake(&mut self) {
        match self.waker.take() {
            Some(waker) => waker.wake(),
            None => {}
        }
    }

    /// Sets the [`Waker`] associated with the event
    pub(super) fn set_waker(&mut self, waker: Option<Waker>) {
        self.waker = waker;
    }

    /// Sets the file descriptor associated with the event, returning the old one
    pub(super) fn set_fd(&mut self, mut fd: Option<c_int>) -> Option<c_int> {
        std::mem::swap(&mut self.file_descriptor, &mut fd);
        fd
    }
}
