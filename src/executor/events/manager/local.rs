use super::EventList;
use std::sync::mpsc::{self, Receiver, Sender};

/// The event manager for an executor on one thread
pub(super) struct LocalEventManager {
    /// The list of active events
    events: EventList,

    /// The queue of events which have been activated
    queue: Receiver<usize>,

    /// The sender onto which events will be queued
    sender: Sender<usize>,
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
    pub(super) fn sender(&self) -> Sender<usize> {
        self.sender.clone()
    }

    /// Registers a new event returning its id
    pub(super) fn register(&mut self) -> usize {
        self.events.insert()
    }

    /// Unregisters an event
    pub(super) fn unregister(&mut self, event: usize) {
        self.events.remove(event);
    }
}
