/// An ID which uniquely identifies an event
#[derive(Clone, Copy)]
pub union EventID {
    id: ID,
    int: u64,
}

/// The individual parts of the ID
#[derive(Clone, Copy, PartialEq, Eq)]
struct ID {
    /// The index into the event list
    index: u32,

    /// The key which this event was assigned
    key: u32,
}

impl EventID {
    /// Creates a new [`EventID`]
    pub(super) fn new(index: usize, key: u32) -> Self {
        EventID {
            id: ID {
                index: index as u32,
                key,
            },
        }
    }

    /// Creates an [`EventID`] from `value`
    pub(super) fn from_u64(value: u64) -> Self {
        EventID { int: value }
    }

    /// Gets the index of this event in the list
    pub(super) fn index(&self) -> usize {
        unsafe { self.id.index as usize }
    }

    /// Gets the key this event was assigned
    pub(super) fn key(&self) -> u32 {
        unsafe { self.id.key }
    }

    pub(super) fn as_u64(&self) -> u64 {
        unsafe { self.int }
    }
}

impl PartialEq for EventID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.id == other.id }
    }
}
