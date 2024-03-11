use executor::{EventID, EventManager, Result};
use std::ops::Deref;
use win32::HANDLE;

/// A container for an [`EventID`] which deregisters on drop
pub(crate) struct EventRef(EventID);

impl EventRef {
    /// Registers a new event with the local event manager and returns an [`EventRef`] to it
    pub(crate) fn register() -> Result<Self> {
        EventManager::get_local_mut(|manager| manager.register(0))
            .map(|event_id| EventRef(event_id))
            .ok_or(win32::Error::new(win32::ERROR_TOO_MANY_CMDS))
    }

    /// Registers a new event associated with a [`HANDLE`] with the local event manager and returns
    /// an [`EventRef`] to it
    pub(crate) fn register_handle(handle: HANDLE) -> Result<Self> {
        EventManager::get_local_mut(|manager| manager.register_handle(0, handle))?
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
