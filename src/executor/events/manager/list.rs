use std::task::Waker;

/// A list of outstanding events
pub(super) struct EventList {
    /// The list
    list: Vec<Node>,

    /// The index of the first free node in `list`
    first_free: Option<usize>,

    /// The number of used nodes in `list`
    len: usize,
}

enum Node {
    /// The node is not registered
    Free {
        /// The index of the next free node in `list`
        next_free: Option<usize>,
    },

    /// The node is registered
    Used(Option<Waker>),
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
    ///
    /// # Panic
    /// This function will panic if `id` has not been inserted into the list
    pub(super) fn get_mut(&mut self, id: usize) -> &mut Option<Waker> {
        match &mut self.list[id] {
            Node::Free { next_free: _ } => {
                panic!("event {} was accessed but has not been registered!", id)
            }
            Node::Used(event) => event,
        }
    }

    /// Inserts a new event into the list, returning the ID to access the event
    pub(super) fn insert(&mut self) -> usize {
        self.len += 1;

        if self.first_free.is_none() {
            return self.insert_at_end();
        }

        let index = self.first_free.unwrap();
        self.first_free = self.list[index].next_free();
        self.list[index] = Node::Used(None);
        index
    }

    /// Removes an event from the list given its `id`
    ///
    /// # Panic
    /// This function will panic if `id` has not been inserted into the list
    pub(super) fn remove(&mut self, id: usize) {
        match self.list[id] {
            Node::Free { next_free: _ } => panic!("Attempting to remove a free node"),
            _ => {}
        }

        self.list[id] = Node::Free {
            next_free: self.first_free,
        };
        self.first_free = Some(id);
    }

    /// Inserts an event at the end of `list`
    fn insert_at_end(&mut self) -> usize {
        self.list.push(Node::Used(None));
        self.list.len() - 1
    }
}

impl Node {
    /// Gets the index of the next free node
    ///
    /// # Panic
    /// This function will panic if the node is not free
    pub(self) fn next_free(&self) -> Option<usize> {
        match self {
            Node::Free { next_free } => *next_free,
            Node::Used(_) => panic!("Attempting to get next_free of used node"),
        }
    }
}
