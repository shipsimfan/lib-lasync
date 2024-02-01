use std::{ops::Deref, pin::Pin, sync::mpsc::Sender};

/// An event waiting for a signal to activate it
pub struct SignalValue {
    /// The ID of the event
    id: usize,

    /// The queue to signal
    sender: Sender<usize>,
}

impl SignalValue {
    /// Creates a new [`SignalValue`]
    pub(in crate::executor) fn new(id: usize, sender: Sender<usize>) -> Pin<Box<Self>> {
        Box::pin(SignalValue { id, sender })
    }

    /// Gets the ID of the event for this signal
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Deref for SignalValue {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}
