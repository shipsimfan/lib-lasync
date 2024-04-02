use super::Task;
use std::{cell::RefCell, collections::VecDeque, future::Future, rc::Rc};

/// A queue of [`Future`]s to complete
#[derive(Clone)]
pub struct FutureQueue<'a>(Rc<RefCell<VecDeque<Rc<Task<'a>>>>>);

impl<'a> FutureQueue<'a> {
    /// Creates a new empty [`FutureQueue`]
    pub fn new() -> Self {
        FutureQueue(Rc::new(RefCell::new(VecDeque::new())))
    }

    /// Gets the number of tasks in the queue
    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    /// Pushes `future` onto the back of the queue
    pub fn push(&self, future: impl Future<Output = ()> + 'a) {
        let task = Rc::new(Task::new(future, self.clone()));
        self.push_raw(task);
    }

    /// Remove the next [`Task`] from the queue
    pub(crate) fn pop(&self) -> Option<Rc<Task<'a>>> {
        self.0.borrow_mut().pop_front()
    }

    /// Push a an already formed [`Task`] onto the queue
    pub(super) fn push_raw(&self, task: Rc<Task<'a>>) {
        self.0.borrow_mut().push_back(task);
    }
}

impl<'a> !Send for FutureQueue<'a> {}
impl<'a> !Sync for FutureQueue<'a> {}
