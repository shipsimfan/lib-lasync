use crate::platform::LocalEventManager;

mod tls;

/// A manager for asynchronous events
pub struct EventManager {
    /// Prevents this structure from being made outside of this module
    _priv: (),
}

impl EventManager {
    /// Creates a new [`EventManager`] for the current thread
    ///
    /// # Panic
    /// This function will panic if another [`EventManager`] has already been created for the
    /// current thread.
    pub(crate) fn new() -> Self {
        tls::get_opt_mut(|manager| {
            if manager.is_some() {
                panic!("Attempted to created a second event manager on a thread");
            }

            *manager = Some(LocalEventManager::new());
        });

        EventManager { _priv: () }
    }

    /// Gets the [`LocalEventManager`] for the current thread
    pub fn get_local<T, F: FnOnce(&LocalEventManager) -> T>(f: F) -> T {
        tls::get(f)
    }

    /// Gets the [`LocalEventManager`] for the current thread mutably
    pub fn get_local_mut<T, F: FnOnce(&mut LocalEventManager) -> T>(f: F) -> T {
        tls::get_mut(f)
    }

    /// Gets the number of outstanding events
    pub(crate) fn len(&self) -> usize {
        todo!("EventManager::len")
    }

    /// Waits for an event to be triggered
    pub(crate) fn poll(&self) {
        todo!("EventManager::poll")
    }
}

impl Drop for EventManager {
    fn drop(&mut self) {
        tls::get_opt_mut(|manager| {
            *manager = None;
        })
    }
}