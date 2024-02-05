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
    pub(in crate::executor::events) fn new(index: usize, key: u32) -> Self {
        EventID {
            id: ID {
                index: index as u32,
                key,
            },
        }
    }

    /// Creates an [`EventID`] from `value`
    pub(in crate::executor::events) fn from_u64(value: u64) -> Self {
        EventID { int: value }
    }

    /// Gets the index of this event in the list
    pub(in crate::executor::events) fn index(&self) -> usize {
        unsafe { self.id.index as usize }
    }

    /// Gets the key this event was assigned
    pub(in crate::executor::events) fn key(&self) -> u32 {
        unsafe { self.id.key }
    }

    pub(in crate::executor::events) fn as_u64(&self) -> u64 {
        unsafe { self.int }
    }
}

impl PartialEq for EventID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.id == other.id }
    }
}

impl std::fmt::Display for EventID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", unsafe { self.id.index }, unsafe { self.id.key })
    }
}
