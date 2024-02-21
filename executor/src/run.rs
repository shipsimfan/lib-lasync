use crate::{platform::Result, EventManager, FutureQueue, WakerRef};
use std::{future::Future, num::NonZeroUsize, task::Context};

/// Runs a local executor on `future`
pub fn run(size: NonZeroUsize, future: impl Future<Output = ()> + 'static) -> Result<()> {
    let queue = FutureQueue::new();
    queue.push(future);
    run_queue(size, queue)
}

/// Executes the tasks in the [`FutureQueue`]
pub fn run_queue(size: NonZeroUsize, queue: FutureQueue) -> Result<()> {
    let mut event_manager = EventManager::new(size)?;

    loop {
        // Drive any tasks that need to be
        while let Some(task) = queue.pop() {
            let mut future_slot = task.future().borrow_mut();
            if let Some(mut future) = future_slot.take() {
                let waker = WakerRef::new(&task);
                let context = &mut Context::from_waker(&waker);

                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }

        // If there are no events being waited on and no tasks to process, there is nothing
        // remaining to drive forward and we are done
        let no_events = event_manager.len() == 0;
        let no_tasks = queue.len() == 0;
        if no_events && no_tasks {
            return Ok(());
        }

        // Wait for events as there are no more tasks to perform
        event_manager.poll()?;
    }
}
