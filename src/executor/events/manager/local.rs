use super::EventList;
use crate::executor::EventID;
use std::task::Waker;

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
    pub(super) fn update(&mut self, event: EventID, waker: Option<Waker>) {
        *self.events.get_mut(event).unwrap() = waker;
    }

    /// Blocks the current thread until an event triggers and wakes any triggered events
    pub(super) fn poll(&mut self) {
        let mut timeout = -1;
        loop {
            // Poll for event

            todo!("Get waker");

            todo!("Call waker");

            todo!("Clear waker");
        }
    }

    /// Unregisters an event
    pub(super) fn unregister(&mut self, event: EventID) {
        self.events.remove(event);
    }
}
