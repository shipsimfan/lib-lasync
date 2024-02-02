use super::EventID;
use linux::signal::{sigevent, sigval, SIGEV_SIGNAL};
use list::EventList;
use local::LocalEventManager;
use std::{pin::Pin, task::Waker};

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

    /// Registers a new event for the current thread and returns the event ID (inside the
    /// [`SignalValue`]) and a [`sigevent`] object for registering the signal callback.
    ///
    /// The [`sigevent`] object points to the [`SignalValue`] so the [`SignalValue`] must live
    /// as long as the event is registered
    pub fn register_signal() -> (Pin<Box<EventID>>, sigevent) {
        let id = Box::pin(tls::get_mut(|manager| manager.register()));

        let sigevent = sigevent {
            notify: SIGEV_SIGNAL,
            signo: todo!("Signal number"),
            value: sigval {
                ptr: &*id as *const _ as _,
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
        tls::get_mut(|manager| manager.update(event, Some(waker)));
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
