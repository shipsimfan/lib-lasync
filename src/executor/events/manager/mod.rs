use super::Signal;
use list::EventList;
use local::LocalEventManager;
use std::ffi::c_int;

mod list;
mod local;
mod tls;

/// The manager for events on the current thread
pub struct EventManager {
    /// Prevents this structure from being made outside of this module
    _priv: (),
}

impl EventManager {
    /// Creates a new [`EventManager`] for the current thread
    ///
    /// # Panic
    /// This function will panic if `signal_number` is not between 32 and 64 inclusive
    pub(in crate::executor) fn new(signal_number: c_int) -> linux::Result<Self> {
        let signal = Signal::register(signal_number)?;

        let local_event_manager = LocalEventManager::new(signal);

        tls::register(local_event_manager);

        Ok(EventManager { _priv: () })
    }

    /// Gets the number of active events
    pub(in crate::executor) fn len(&self) -> usize {
        tls::len()
    }
}

impl Drop for EventManager {
    fn drop(&mut self) {
        tls::unregister();
    }
}
