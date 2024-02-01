use super::EventList;
use crate::executor::EventID;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    task::Waker,
};

/// The event manager for an executor on one thread
pub(super) struct LocalEventManager {
    /// The list of active events
    events: EventList,

    /// The queue of events which have been activated
    queue: Receiver<EventID>,

    /// The sender onto which events will be queued
    sender: Sender<EventID>,
}

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`]
    pub(super) fn new() -> Self {
        let (sender, queue) = mpsc::channel();

        LocalEventManager {
            events: EventList::new(),
            queue,
            sender,
        }
    }

    /// Gets the number of active events
    pub(super) fn len(&self) -> usize {
        self.events.len()
    }

    /// Gets the sender which is used to trigger events
    pub(super) fn sender(&self) -> Sender<EventID> {
        self.sender.clone()
    }

    /// Registers a new event returning its id
    pub(super) fn register(&mut self) -> EventID {
        self.events.insert()
    }

    /// Sets the [`Waker`] associated with `event`
    ///
    /// # Panic
    /// This function will panic if `event` is not registered
    pub(super) fn update(&mut self, event: EventID, waker: Option<Waker>) {
        *self.events.get_mut(event).unwrap() = waker;
    }

    /// Unregisters an event
    pub(super) fn unregister(&mut self, event: EventID) {
        self.events.remove(event);
    }
}
