use super::Task;
use std::{
    future::Future,
    sync::{mpsc::SyncSender, Arc},
};

/// A queue of [`Future`]s to complete
#[derive(Clone)]
pub struct FutureQueue(SyncSender<Arc<Task>>);

impl FutureQueue {
    /// Creates a new [`FutureQueue`] from `sender`
    pub(super) fn new(sender: SyncSender<Arc<Task>>) -> Self {
        FutureQueue(sender)
    }

    /// Pushes `future` onto the back of the queue
    pub fn push(&self, future: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Task::new(future, self.clone()));
        self.push_raw(task);
    }

    pub(super) fn push_raw(&self, task: Arc<Task>) {
        self.0.send(task).unwrap();
    }
}
