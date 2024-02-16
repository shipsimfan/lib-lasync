use executor::{EventID, EventManager};
use std::ops::Deref;

/// A container for an [`EventID`] which deregisters on drop
pub(crate) struct EventRef(EventID);

impl EventRef {
    /// Creates a new [`EventRef`]
    pub(crate) fn new(event_id: EventID) -> Self {
        EventRef(event_id)
    }
}

impl Deref for EventRef {
    type Target = EventID;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for EventRef {
    fn drop(&mut self) {
        EventManager::get_local_mut(|manager| manager.deregister(self.0));
    }
}
