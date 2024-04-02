use crate::{platform::LocalEventManager, Result};
use std::num::NonZeroUsize;

mod tls;

/// A manager for asynchronous events
pub struct EventManager {
    /// Prevents this structure from being made outside of this module
    _priv: (),
}

impl EventManager {
    /// Creates a new [`EventManager`] for the current thread that can handle at most `size`
    /// simultaneous events
    ///
    /// # Panic
    /// This function will panic if another [`EventManager`] has already been created for the
    /// current thread.
    pub(crate) fn new(size: NonZeroUsize) -> Result<Self> {
        tls::get_opt_mut(|manager| {
            if manager.is_some() {
                panic!("Attempted to created a second event manager on a thread");
            }

            *manager = Some(LocalEventManager::new(size)?);

            Ok(EventManager { _priv: () })
        })
    }

    /// Gets the [`LocalEventManager`] for the current thread
    pub fn get_local<T, F: FnOnce(&LocalEventManager) -> T>(f: F) -> T {
        tls::get(f)
    }

    /// Gets the [`LocalEventManager`] for the current thread mutably
    pub fn get_local_mut<T, F: FnOnce(&mut LocalEventManager) -> T>(f: F) -> T {
        tls::get_mut(f)
    }

    /// Gets the [`LocalEventManager`] for the current thread mutably without checking borrow counts
    ///
    /// # Saftey
    /// This must only be used where it can be garunteed that there will not be multiple concurrent
    /// mutable references to the event manager
    pub unsafe fn get_unchecked_mut<T, F: FnOnce(&mut LocalEventManager) -> T>(f: F) -> T {
        tls::get_unchecked_mut(f)
    }

    /// Gets the number of outstanding events
    pub(crate) fn len(&self) -> usize {
        tls::get(|manager| manager.len())
    }

    /// Waits for an event to be triggered
    pub(crate) fn poll(&mut self) -> Result<()> {
        tls::get_mut(|manager| manager.poll())
    }
}

impl Drop for EventManager {
    fn drop(&mut self) {
        tls::get_opt_mut(|manager| {
            *manager = None;
        })
    }
}

impl !Send for EventManager {}
impl !Sync for EventManager {}
