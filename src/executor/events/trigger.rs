use super::EventID;
use std::{ops::Deref, pin::Pin, sync::mpsc::Sender};

/// An event waiting for a signal to activate it
pub struct EventTrigger {
    /// The ID of the event
    id: EventID,

    /// The queue to signal
    sender: Sender<EventID>,
}

impl EventTrigger {
    /// Creates a new [`EventTrigger`]
    pub(in crate::executor) fn new(id: EventID, sender: Sender<EventID>) -> Pin<Box<Self>> {
        Box::pin(EventTrigger { id, sender })
    }

    /// Gets the ID of the event
    pub fn id(&self) -> EventID {
        self.id
    }
}

impl Deref for EventTrigger {
    type Target = EventID;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
