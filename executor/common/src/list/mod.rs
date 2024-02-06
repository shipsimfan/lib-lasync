use crate::EventID;
use node::Node;
use std::ops::{Index, IndexMut};

mod node;

/// A list of items
pub struct List<T> {
    /// The list of nodes with their current keys
    list: Box<[Node<T>]>,

    /// The index of the first free node in `list`
    first_free: Option<usize>,

    /// The number of used nodes in `list`
    len: usize,
}

impl<T> List<T> {
    /// Creates a new empty [`List`] that can hold at most `size` elements
    ///
    /// # Panic
    /// This function will panic if `size` is zero
    pub fn new(size: usize) -> Self {
        assert_ne!(size, 0);

        let mut list = Vec::with_capacity(size);
        for i in 0..size {
            let next_free = if i + 1 < size { Some(i + 1) } else { None };

            list.push(Node::new(next_free));
        }

        List {
            list: list.into_boxed_slice(),
            first_free: Some(0),
            len: 0,
        }
    }

    /// Gets the number of items in the list
    pub fn len(&self) -> usize {
        self.len
    }

    /// Gets an item with `id`
    pub fn get(&self, id: EventID) -> Option<&T> {
        self.get_node(id)?.item()
    }

    /// Gets an item with `id` mutably
    pub fn get_mut(&mut self, id: EventID) -> Option<&mut T> {
        self.get_node_mut(id)?.item_mut()
    }

    /// Inserts a new item into the list, returning the ID to access the item if there is enough
    /// room available
    pub fn insert(&mut self, item: T) -> Option<EventID> {
        if self.first_free.is_none() {
            return None;
        }

        self.len += 1;

        let index = self.first_free.unwrap();
        let (key, new_first_free) = self.list[index].set_used(item);
        self.first_free = new_first_free;

        Some(EventID::new(index, key))
    }

    /// Removes an item from the list given its `id`
    ///
    /// Returns the removed item
    pub fn remove(&mut self, id: EventID) -> Option<T> {
        let first_free = self.first_free;

        let node = self.get_node_mut(id)?;

        let item = node.set_free(first_free);
        self.first_free = Some(id.index());
        self.len -= 1;

        Some(item)
    }

    /// Gets a node with `id` if it is used
    fn get_node(&self, id: EventID) -> Option<&Node<T>> {
        let node = self.list.get(id.index())?;
        if !node.key_matches(id.key()) {
            return None;
        }

        Some(node)
    }

    /// Mutably gets a node with `id` if it is used
    fn get_node_mut(&mut self, id: EventID) -> Option<&mut Node<T>> {
        let node = self.list.get_mut(id.index())?;
        if !node.key_matches(id.key()) {
            return None;
        }

        Some(node)
    }
}

impl<T> Index<EventID> for List<T> {
    type Output = T;

    fn index(&self, index: EventID) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<EventID> for List<T> {
    fn index_mut(&mut self, index: EventID) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
