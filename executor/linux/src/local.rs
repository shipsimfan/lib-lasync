use super::{EPoll, EventList};
use crate::executor::{events::Signal, EventID};
use linux::sys::epoll::EPOLLIN;
use std::{ffi::c_int, task::Waker};

/// The event manager for an executor on one thread
pub(super) struct LocalEventManager {
    /// The list of active events
    events: EventList,

    /// The epoll object to poll events with
    epoll: EPoll,

    /// The signal file descriptor to poll for signalled events
    signal: Signal,
}

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`]
    pub(super) fn new(signal_number: c_int) -> linux::Result<Self> {
        let events = EventList::new();
        let signal = Signal::new(signal_number)?;

        let mut epoll = EPoll::new()?;
        epoll.register_fd(signal.fd(), EPOLLIN, u64::MAX)?;

        Ok(LocalEventManager {
            events,
            epoll,
            signal,
        })
    }

    /// Gets the number of active events
    pub(super) fn len(&self) -> usize {
        self.events.len()
    }

    /// Gets the signal number this event manager listens on
    pub(super) fn signal_number(&self) -> c_int {
        self.signal.signal_number()
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

            if event_id == u64::MAX {
                self.wake_signal_events()?;
            } else {
                self.wake_event(EventID::from_u64(event_id));
            }
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

    /// Wake all events waiting from the signal
    fn wake_signal_events(&mut self) -> linux::Result<()> {
        while let Some(event_id) = self.signal.read() {
            self.wake_event(event_id)
        }

        Ok(())
    }

    /// Wakes an event from an `event_id`
    fn wake_event(&mut self, event_id: EventID) {
        match self.events.get_mut(event_id) {
            Some(event) => event.wake(),
            None => panic!("No event"),
        };
    }
}
