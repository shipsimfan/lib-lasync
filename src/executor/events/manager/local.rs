use super::{EPoll, EventList};
use crate::executor::EventID;
use std::{ffi::c_int, task::Waker};

/// The event manager for an executor on one thread
pub(super) struct LocalEventManager {
    /// The list of active events
    events: EventList,

    epoll: EPoll,
}

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`]
    pub(super) fn new() -> linux::Result<Self> {
        let epoll = EPoll::new()?;
        let events = EventList::new();

        Ok(LocalEventManager { events, epoll })
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
    pub(super) fn set_fd(
        &mut self,
        event: EventID,
        fd: Option<c_int>,
        events: u32,
    ) -> linux::Result<()> {
        if let Some(old_fd) = self.events[event].set_fd(fd) {
            self.epoll.unregister_fd(old_fd)?;
        }

        if let Some(fd) = fd {
            self.epoll.register_fd(fd, events, event.as_u64())?;
        }

        Ok(())
    }

    /// Blocks the current thread until an event triggers and wakes any triggered events
    pub(super) fn poll(&mut self) -> linux::Result<()> {
        let mut block = true;
        while let Some(event_id) = self.epoll.poll(block)? {
            block = false;
            let event_id = EventID::from_u64(event_id);

            let event = match self.events.get_mut(event_id) {
                Some(event) => event,
                None => continue,
            };

            event.wake();
        }

        Ok(())
    }

    /// Unregisters an event
    pub(super) fn unregister(&mut self, event: EventID) -> linux::Result<()> {
        let event = match self.events.remove(event) {
            Some(event) => event,
            None => return Ok(()),
        };

        if let Some(fd) = event.fd() {
            self.epoll.unregister_fd(fd)?;
        }

        Ok(())
    }
}
