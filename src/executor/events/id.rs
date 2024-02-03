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
    pub(super) fn new(index: usize, key: usize) -> Self {
        EventID {
            id: ID {
                index: index as u32,
                key: key as u32,
            },
        }
    }

    /// Gets the index of this event in the list
    pub(super) fn index(&self) -> usize {
        unsafe { self.id.index as usize }
    }

    /// Gets the key this event was assigned
    pub(super) fn key(&self) -> usize {
        unsafe { self.id.key as usize }
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
