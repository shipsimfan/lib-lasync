use std::fmt::Display;

/// An ID which uniquely identifies an event
#[repr(align(8))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EventID {
    /// The index into the event list
    index: u32,

    /// The key which this event was assigned
    key: u32,
}

impl EventID {
    /// Creates a new [`EventID`]
    pub(crate) fn new(index: usize, key: u32) -> Self {
        EventID {
            index: index as u32,
            key,
        }
    }

    /// Creates an [`EventID`] from `value`
    pub unsafe fn from_u64(value: u64) -> Self {
        std::mem::transmute(value)
    }

    /// Gets the index of this event in the list
    pub fn index(&self) -> usize {
        self.index as usize
    }

    /// Gets the key this event was assigned
    pub fn key(&self) -> u32 {
        self.key
    }

    /// Converts this value into a [`u64`]
    pub fn into_u64(self) -> u64 {
        unsafe { std::mem::transmute(self) }
    }
}

impl Display for EventID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.index, self.key)
    }
}
