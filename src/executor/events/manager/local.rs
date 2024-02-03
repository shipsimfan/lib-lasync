use super::EventList;
use crate::executor::EventID;
use std::{ffi::c_int, task::Waker};

/// The event manager for an executor on one thread
pub(super) struct LocalEventManager {
    /// The list of active events
    events: EventList,
}

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`]
    pub(super) fn new() -> Self {
        LocalEventManager {
            events: EventList::new(),
        }
    }

    /// Gets the number of active events
    pub(super) fn len(&self) -> usize {
        self.events.len()
    }

    /// Registers a new event returning its id
    pub(super) fn register(&mut self) -> EventID {
        self.events.insert()
    }

    /// Sets the [`Waker`] associated with `event`
    ///
    /// # Panic
    /// This function will panic if `event` is not registered
    pub(super) fn set_waker(&mut self, event: EventID, waker: Option<Waker>) {
        self.events[event].set_waker(waker);
    }

    /// Sets a file descriptor associated with an event
    pub(super) fn set_fd(&mut self, event: EventID, fd: Option<c_int>) {
        if let Some(old_fd) = self.events[event].set_fd(fd) {
            todo!("Unregister with epoll");
        }

        if let Some(fd) = fd {
            todo!("Register with epoll");
        }
    }

    /// Blocks the current thread until an event triggers and wakes any triggered events
    pub(super) fn poll(&mut self) {
        let mut timeout = -1;
        loop {
            todo!("Poll from epoll");

            todo!("Get waker");

            todo!("Call waker");

            todo!("Clear waker");
        }
    }

    /// Unregisters an event
    pub(super) fn unregister(&mut self, event: EventID) {
        let event = match self.events.remove(event) {
            Some(event) => event,
            None => return,
        };

        if let Some(fd) = event.fd() {
            todo!("Unregister with epoll");
        }
    }
}
