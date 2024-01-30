use std::sync::mpsc::Sender;

/// An event waiting for a signal to activate it
pub struct SignalValue {
    /// The ID of the event
    id: usize,

    /// The queue to signal
    sender: Sender<usize>,
}
