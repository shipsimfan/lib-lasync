use linux::signal::{sigevent, sigval, SIGEV_SIGNAL};
use local::LocalEventManager;
use std::{ffi::c_int, task::Waker};

/// The manager for events on the current thread
pub struct EventManager {
    /// Prevents this structure from being made outside of this module
    _priv: (),
}

impl EventManager {
    /// Creates a new [`EventManager`] for the current thread
    pub(in crate::executor) fn new(signal_number: c_int) -> linux::Result<Self> {
        let local_event_manager = LocalEventManager::new(signal_number)?;

        tls::register(local_event_manager);

        Ok(EventManager { _priv: () })
    }

    /// Gets the number of active events
    pub(in crate::executor) fn len(&self) -> usize {
        tls::get(|manager| manager.len())
    }

    /// Blocks the current thread until an event triggers and wakes any triggered events
    pub(in crate::executor) fn poll(&self) -> linux::Result<()> {
        tls::get_mut(|manager| manager.poll())
    }

    /// Registers a new event for the current thread and returns the event ID
    pub fn register() -> EventID {
        tls::get_mut(|manager| manager.register())
    }

    /// Registers a new event for the current thread and registers a file descriptor for
    /// monitoring. This function returns the [`EventID`] for the new event.
    pub fn register_fd(fd: c_int, events: u32) -> linux::Result<EventID> {
        tls::get_mut(|manager| {
            let id = manager.register();
            manager.set_fd(id, Some(fd), events).map(|_| id)
        })
    }

    /// Registers a new event for the current thread and returns the [`EventID`] and a [`sigevent`]
    /// object for registering the signal callback.
    pub fn register_signal() -> (EventID, sigevent) {
        let (id, signo) = tls::get_mut(|manager| (manager.register(), manager.signal_number()));

        let sigevent = sigevent {
            notify: SIGEV_SIGNAL,
            signo,
            value: sigval {
                ptr: id.as_u64() as _,
            },
            ..Default::default()
        };

        (id, sigevent)
    }

    /// Sets the [`Waker`] called when `event` is triggered
    ///
    /// # Panic
    /// This function will panic if `event` is not registered
    pub fn set_waker(event: EventID, waker: Waker) {
        tls::get_mut(|manager| manager.set_waker(event, Some(waker)));
    }

    /// Unregisters an event for the current thread
    pub fn unregister(event: EventID) -> linux::Result<()> {
        tls::get_mut(|manager| manager.unregister(event))
    }
}

impl Drop for EventManager {
    fn drop(&mut self) {
        tls::unregister();
    }
}
