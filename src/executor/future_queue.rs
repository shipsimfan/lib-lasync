use super::Task;
use std::{cell::RefCell, collections::VecDeque, future::Future, rc::Rc};

/// A queue of [`Future`]s to complete
pub struct FutureQueue<T>(Rc<RefCell<VecDeque<Rc<Task<T>>>>>);

impl<T> FutureQueue<T> {
    /// Creates a new empty [`FutureQueue`]
    pub(super) fn new() -> Self {
        FutureQueue(Rc::new(RefCell::new(VecDeque::new())))
    }

    /// Pushes `future` onto the back of the queue
    pub fn push(&self, future: impl Future<Output = T> + 'static) {
        let task = Rc::new(Task::new(future, self.clone()));
        self.push_raw(task);
    }

    pub(super) fn push_raw(&self, task: Rc<Task<T>>) {
        self.0.borrow_mut().push_back(task);
    }

    pub(super) fn pop(&self) -> Option<Rc<Task<T>>> {
        self.0.borrow_mut().pop_front()
    }
}

impl<T> Clone for FutureQueue<T> {
    fn clone(&self) -> Self {
        FutureQueue(self.0.clone())
    }
}
