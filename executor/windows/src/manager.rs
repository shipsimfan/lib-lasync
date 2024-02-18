use crate::Result;
use executor_common::{Event, EventID, List, Pollable};
use std::num::NonZeroUsize;
use win32::{SleepEx, INFINITE, TRUE};

/// The manager of events on a thread
pub struct LocalEventManager {
    events: List<Event<usize>>,
}

/// Allows calling [`SleepEx`] while not holding the reference to the [`LocalEventManager`].
///
/// This is required because the APCs for events run during the call to [`SleepEx`] and directly
/// wake the events. If the [`LocalEventManager`] was held, the program would panic.
pub struct SleepPoll;

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`] with space for at most `size` simultaneous events
    pub fn new(size: NonZeroUsize) -> Result<Self> {
        let events = List::new(size);

        Ok(LocalEventManager { events })
    }

    /// Gets the number of outstanding events
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Gets an event
    pub fn get_event(&self, event_id: EventID) -> Option<&Event<usize>> {
        self.events.get(event_id)
    }

    /// Registers a new event, returning the [`EventID`] if there is enough room
    pub fn register(&mut self, initial_value: usize) -> Option<EventID> {
        self.events.insert(Event::new(initial_value))
    }

    /// Mutably gets an event
    pub fn get_event_mut(&mut self, event_id: EventID) -> Option<&mut Event<usize>> {
        self.events.get_mut(event_id)
    }

    /// Deregisters an event based on its [`EventID`]
    pub fn deregister(&mut self, event_id: EventID) {
        self.events.remove(event_id);
    }

    /// Sleeps until an event is triggered
    ///
    /// This function returns a [`SleepPoll`] because the event manager's [`RefCell`] cannot be
    /// held. This is because the individual APCs will call the wakers through this event manager
    /// during the poll.
    pub fn poll(&mut self) -> SleepPoll {
        SleepPoll
    }
}

impl Pollable for SleepPoll {
    fn poll(&self) {
        unsafe { SleepEx(INFINITE, TRUE) };
    }
}
