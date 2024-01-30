use super::EventList;
use crate::executor::events::Signal;
use std::sync::mpsc::{self, Receiver, Sender};

/// The event manager for an executor on one thread
pub(super) struct LocalEventManager {
    /// The list of active events
    events: EventList,

    /// The queue of events which have been activated
    queue: Receiver<usize>,

    /// The sender onto which events will be queued
    sender: Sender<usize>,

    /// The signal that triggers this event manager
    signal: Signal,
}

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`]
    pub(super) fn new(signal: Signal) -> Self {
        let (sender, queue) = mpsc::channel();

        LocalEventManager {
            events: EventList::new(),
            queue,
            sender,
            signal,
        }
    }

    /// Gets the number of active events
    pub(super) fn len(&self) -> usize {
        self.events.len()
    }
}
