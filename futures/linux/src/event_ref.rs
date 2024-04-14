use executor::{platform::EventHandler, Error, EventID, EventManager, Result};
use std::ops::Deref;

/// A container for an [`EventID`] which deregisters on drop
pub(crate) struct EventRef(EventID);

impl EventRef {
    /// Registers a new event with the local event manager and returns an [`EventRef`] to it
    pub(crate) fn register(handler: EventHandler) -> Result<Self> {
        EventManager::get_local_mut(|manager| manager.register(handler))
            .map(|event_id| EventRef(event_id))
            .ok_or(Error::ENOSPC)
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
