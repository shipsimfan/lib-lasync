use crate::Result;
use executor::{EventID, EventManager};
use std::ops::Deref;

/// A container for an [`EventID`] which deregisters on drop
pub(crate) struct EventRef(EventID);

impl EventRef {
    /// Registers a new event with the local event manager and returns an [`EventRef`] to it
    pub(crate) fn register() -> Result<Self> {
        EventManager::get_local_mut(|manager| manager.register(0))
            .map(|event_id| EventRef(event_id))
            .ok_or(win32::Error::new(win32::ERROR_TOO_MANY_CMDS))
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
