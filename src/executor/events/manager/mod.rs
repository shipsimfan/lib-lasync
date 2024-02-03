use super::EventID;
use event::Event;
use linux::signal::{sigevent, sigval, SIGEV_SIGNAL};
use list::EventList;
use local::LocalEventManager;
use std::{ffi::c_int, pin::Pin, task::Waker};

mod event;
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
    pub(in crate::executor) fn new() -> linux::Result<Self> {
        let local_event_manager = LocalEventManager::new();

        tls::register(local_event_manager);

        Ok(EventManager { _priv: () })
    }

    /// Gets the number of active events
    pub(in crate::executor) fn len(&self) -> usize {
        tls::get(|manager| manager.len())
    }

    /// Blocks the current thread until an event triggers and wakes any triggered events
    pub(in crate::executor) fn poll(&self) {
        tls::get_mut(|manager| manager.poll())
    }

    /// Registers a new event for the current thread and returns the event ID
    pub fn register() -> EventID {
        tls::get_mut(|manager| manager.register())
    }

    /// Registers a new event for the current thread and registers a file descriptor for
    /// monitoring. This function returns the [`EventID`] for the new event.
    pub fn register_fd(fd: c_int) -> EventID {
        tls::get_mut(|manager| {
            let id = manager.register();
            manager.set_fd(id, Some(fd));
            id
        })
    }

    /// Registers a new event for the current thread and returns the [`EventID`] and a [`sigevent`]
    /// object for registering the signal callback.
    pub fn register_signal() -> (EventID, sigevent) {
        let id = Self::register();

        let sigevent = sigevent {
            notify: SIGEV_SIGNAL,
            value: sigval {
                ptr: id.as_u64() as _,
            },
            signo: todo!("Signal number"),
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
    pub fn unregister(event: EventID) {
        tls::get_mut(|manager| manager.unregister(event))
    }
}

impl Drop for EventManager {
    fn drop(&mut self) {
        tls::unregister();
    }
}
