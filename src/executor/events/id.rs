/// An ID which uniquely identifies an event
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EventID {
    /// The index into the event list
    index: usize,

    /// The key which this event was assigned
    key: usize,
}

impl EventID {
    /// Creates a new [`EventID`]
    pub(super) fn new(index: usize, key: usize) -> Self {
        EventID { index, key }
    }

    /// Gets the index of this event in the list
    pub(super) fn index(&self) -> usize {
        self.index
    }

    /// Gets the key this event was assigned
    pub(super) fn key(&self) -> usize {
        self.key
    }
}
