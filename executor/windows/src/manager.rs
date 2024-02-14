use executor_common::{Event, List, Pollable};
use std::num::NonZeroUsize;
use win32::{SleepEx, INFINITE, TRUE};

/// The manager of events on a thread
pub struct LocalEventManager {
    events: List<Event>,
}

pub struct SleepPoll;

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`] with space for at most `size` simultaneous events
    pub fn new(size: NonZeroUsize) -> Self {
        let events = List::new(size);

        LocalEventManager { events }
    }

    /// Gets the number of outstanding events
    pub fn len(&self) -> usize {
        self.events.len()
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
