use event::Event;
use inner::EventManagerInner;
use std::{cell::RefCell, ffi::c_int, task::Waker};

mod event;
mod inner;

/// An event manager on one thread
pub struct EventManager {
    /// Prevents the struct from being created elsewhere
    _priv: (),
}

thread_local! {
    static LOCAL_EVENT_MANAGER: RefCell<Option<EventManagerInner>> = RefCell::new(None);
}

impl EventManager {
    /// Creates an [`EventManager`] for the current thread
    ///
    /// # Panic
    /// This function will panic if another [`EventManager`] exists on the current thread
    pub(super) fn new() -> linux::Result<EventManager> {
        LOCAL_EVENT_MANAGER.with(|local_event_manager| {
            let mut local_event_manager = local_event_manager.borrow_mut();

            if local_event_manager.is_some() {
                panic!("Cannot run multiple lasync executors at the same time on one thread!");
            }

            *local_event_manager = Some(EventManagerInner::new()?);

            Ok(EventManager { _priv: () })
        })
    }

    /// (Re)registers an event to be polled
    pub fn register(file_descriptor: c_int, events: u32, waker: Waker) -> linux::Result<()> {
        LOCAL_EVENT_MANAGER.with(|local_event_manager| {
            match &mut *local_event_manager.borrow_mut() {
                Some(local_event_manager) => {
                    local_event_manager.register(file_descriptor, events, waker)
                }
                None => panic!("Cannot register an event without a local executor running"),
            }
        })
    }

    /// Stops polling for an event
    pub fn unregister(file_descriptor: c_int) -> linux::Result<()> {
        LOCAL_EVENT_MANAGER.with(|local_event_manager| {
            match &mut *local_event_manager.borrow_mut() {
                Some(local_event_manager) => {
                    local_event_manager.unregister(file_descriptor).map(|_| ())
                }
                None => panic!("Cannot unregister an event without a local executor running"),
            }
        })
    }

    /// Gets the number of events currently being polled
    pub(super) fn count(&self) -> usize {
        LOCAL_EVENT_MANAGER.with(|local_event_manager| match &*local_event_manager.borrow() {
            Some(local_event_manager) => local_event_manager.count(),
            None => unreachable!(),
        })
    }

    /// Blocks until an event becomes ready and wakes the ready events
    pub(super) fn poll(&self) -> linux::Result<()> {
        LOCAL_EVENT_MANAGER.with(|local_event_manager| {
            match &mut *local_event_manager.borrow_mut() {
                Some(local_event_manager) => local_event_manager.poll(),
                None => unreachable!(),
            }
        })
    }
}

impl Drop for EventManager {
    fn drop(&mut self) {
        LOCAL_EVENT_MANAGER.with(|local_event_manager| {
            *local_event_manager.borrow_mut() = None;
        });
    }
}
