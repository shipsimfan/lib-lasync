use std::{collections::VecDeque, task::Waker};

/// A list of tasks waiting to queued
pub struct WaitQueue(VecDeque<Waker>);

impl WaitQueue {
    /// Creates a new empty [`WaitQueue`]
    pub const fn new() -> Self {
        WaitQueue(VecDeque::new())
    }

    /// Gets the number of tasks waiting in this queue
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Pushes `waker` to the back of the queue
    pub fn push(&mut self, waker: Waker) {
        self.0.push_back(waker);
    }

    /// Pops off the next waiting task if there is one
    pub fn pop(&mut self) -> Option<Waker> {
        self.0.pop_front()
    }
}

impl !Send for WaitQueue {}
impl !Sync for WaitQueue {}
