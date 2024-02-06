/// An allocated node in the list
pub(super) enum Node<T> {
    /// The node is not registered
    Free {
        /// The key for the next event in this node
        next_key: u32,

        /// The index of the next free node in `list`
        next_free: Option<usize>,
    },

    /// The node is registered
    Used {
        /// The key identifying this event
        key: u32,

        /// The contained item
        item: T,
    },
}

impl<T> Node<T> {
    /// Creates a new [`Node::Free`] with `next_key` set to 0
    pub(super) const fn new(next_free: Option<usize>) -> Self {
        Node::Free {
            next_key: 0,
            next_free,
        }
    }

    /// Gets the item contained in this node, if there is one
    pub(super) fn item(&self) -> Option<&T> {
        match self {
            Node::Used { key: _, item } => Some(item),
            Node::Free {
                next_key: _,
                next_free: _,
            } => None,
        }
    }

    /// Checks if `key` matches this nodes key
    ///
    /// `Free` nodes don't have a key so will never match
    pub(super) fn key_matches(&self, key: u32) -> bool {
        match self {
            Node::Used {
                key: node_key,
                item: _,
            } => *node_key == key,
            Node::Free {
                next_key: _,
                next_free: _,
            } => false,
        }
    }

    /// Sets this node to used containing `item` and returns the key and the index of the next free
    /// node
    ///
    /// # Panic
    /// This function will panic if the node is not free
    pub(super) fn set_used(&mut self, item: T) -> (u32, Option<usize>) {
        let (key, next_free) = match self {
            Node::Free {
                next_key,
                next_free,
            } => (*next_key, *next_free),
            Node::Used { key: _, item: _ } => {
                panic!("Attempted to set a used node to used")
            }
        };

        *self = Node::Used { key, item };
        (key, next_free)
    }

    /// Sets this node to free and returns the item that was contained
    ///
    /// # Panic
    /// This function will panic if the node is not used
    pub(super) fn set_free(&mut self, next_free: Option<usize>) -> T {
        let key = match self {
            Node::Used { key, item: _ } => *key,
            Node::Free {
                next_key: _,
                next_free: _,
            } => panic!("Attempted to set a free node to free"),
        };

        let mut new_node = Node::Free {
            next_key: key.wrapping_add(1),
            next_free,
        };
        std::mem::swap(self, &mut new_node);

        match new_node {
            Node::Used { key: _, item } => item,
            Node::Free {
                next_key: _,
                next_free: _,
            } => unreachable!(),
        }
    }

    /// Mutably gets the item contained in this node, if there is one
    pub(super) fn item_mut(&mut self) -> Option<&mut T> {
        match self {
            Node::Used { key: _, item } => Some(item),
            Node::Free {
                next_key: _,
                next_free: _,
            } => None,
        }
    }
}
