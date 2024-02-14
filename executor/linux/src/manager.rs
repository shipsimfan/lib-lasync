use executor_common::{Event, List};
use std::num::NonZeroUsize;

/// The manager of events on a thread
pub struct LocalEventManager {
    events: List<Event>,
}

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`] with space for at most `size` simultaneous events
    ///
    /// # Panic
    /// This function will panic if `size` is over 8192
    pub fn new(size: NonZeroUsize) -> Self {
        assert!(size.get() <= 8192);

        let events = List::new(size);

        LocalEventManager { events }
    }

    /// Gets the number of outstanding events
    pub fn len(&self) -> usize {
        self.events.len()
    }
}
