use std::task::Waker;

use crate::executor::EventID;

/// A list of outstanding events
pub(super) struct EventList {
    /// The list of nodes with their current keys
    list: Vec<Node>,

    /// The index of the first free node in `list`
    first_free: Option<usize>,

    /// The number of used nodes in `list`
    len: usize,
}

enum Node {
    /// The node is not registered
    Free {
        /// The key for the next event in this node
        next_key: usize,

        /// The index of the next free node in `list`
        next_free: Option<usize>,
    },

    /// The node is registered
    Used {
        /// The key identifying this event
        key: usize,

        /// The [`Waker`] used to enqueue the task
        waker: Option<Waker>,
    },
}

impl EventList {
    /// Creates a new empty [`EventList`]
    pub(super) fn new() -> Self {
        EventList {
            list: Vec::new(),
            first_free: None,
            len: 0,
        }
    }

    /// Gets the number of active events in the list
    pub(super) fn len(&self) -> usize {
        self.len
    }

    /// Gets an event with `id`
    pub(super) fn get_mut(&mut self, id: EventID) -> Option<&mut Option<Waker>> {
        let node = &mut self.list[id.index()];
        if node.key() != id.key() {
            return None;
        }

        match node {
            Node::Free {
                next_key: _,
                next_free: _,
            } => None,
            Node::Used { key: _, waker } => Some(waker),
        }
    }

    /// Inserts a new event into the list, returning the ID to access the event
    pub(super) fn insert(&mut self) -> EventID {
        self.len += 1;

        if self.first_free.is_none() {
            return self.insert_at_end();
        }

        let index = self.first_free.unwrap();

        self.first_free = self.list[index].next_free();
        let key = self.list[index].key();

        self.list[index] = Node::Used { key, waker: None };

        EventID::new(index, key)
    }

    /// Removes an event from the list given its `id`
    ///
    /// Returns if the item was removed from the list
    pub(super) fn remove(&mut self, id: EventID) -> bool {
        match self.list[id.index()] {
            Node::Free {
                next_key: _,
                next_free: _,
            } => return false,
            Node::Used { key, waker: _ } => {
                if key != id.key() {
                    return false;
                }
            }
        }

        self.list[id.index()] = Node::Free {
            next_key: id.key() + 1,
            next_free: self.first_free,
        };
        self.first_free = Some(id.index());
        true
    }

    /// Inserts an event at the end of `list`
    fn insert_at_end(&mut self) -> EventID {
        self.list.push(Node::Used {
            key: 0,
            waker: None,
        });
        EventID::new(self.list.len() - 1, 0)
    }
}

impl Node {
    /// Gets the index of the next free node
    ///
    /// # Panic
    /// This function will panic if the node is not free
    pub(self) fn next_free(&self) -> Option<usize> {
        match self {
            Node::Free {
                next_key: _,
                next_free,
            } => *next_free,
            Node::Used { key: _, waker: _ } => panic!("Attempting to get next_free of used node"),
        }
    }

    /// Gets the key for this event if it used or the next key if it is free
    pub(self) fn key(&self) -> usize {
        match self {
            Node::Free {
                next_key,
                next_free: _,
            } => *next_key,
            Node::Used { key, waker: _ } => *key,
        }
    }
}
