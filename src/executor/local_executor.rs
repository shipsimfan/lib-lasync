use super::{FutureQueue, Task};
use std::{
    sync::{
        mpsc::{sync_channel, Receiver},
        Arc,
    },
    task::Context,
};

/// A single-threaded executor
pub struct LocalExecutor {
    /// The queue of [`Task`]s to complete
    queue: Receiver<Arc<Task>>,
}

impl LocalExecutor {
    /// Creates a new [`LocalExecutor`] and [`FutureQueue`] to schedule with
    pub fn new() -> (Self, FutureQueue) {
        let (sender, queue) = sync_channel(1024);
        (LocalExecutor { queue }, FutureQueue::new(sender))
    }

    /// Executes the tasks in the [`FutureQueue`]
    pub fn run(&self) {
        while let Ok(task) = self.queue.recv() {
            let mut future_slot = task.future().lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = task.waker();
                let context = &mut Context::from_waker(&waker);

                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}
