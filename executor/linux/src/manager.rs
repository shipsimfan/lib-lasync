use crate::{IOURing, Result};
use executor_common::{Event, List};
use std::num::NonZeroUsize;

/// The manager of events on a thread
pub struct LocalEventManager {
    events: List<Event>,

    io_uring: IOURing,
}

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`] with space for at most `size` simultaneous events
    ///
    /// # Panic
    /// This function will panic if `size` is over 8192
    pub fn new(size: NonZeroUsize) -> Result<Self> {
        assert!(size.get() <= 8192);

        let events = List::new(size);

        let io_uring = IOURing::new((size.get() / 2) as _)?;

        Ok(LocalEventManager { events, io_uring })
    }

    /// Gets the number of outstanding events
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Sleeps until an event is triggered
    pub fn poll(&mut self) {
        todo!("poll");
    }
}
