//! Single-threaded executor for async/await

use std::{ffi::c_int, future::Future, task::Context};
use task::Task;
use waker::WakerRef;

mod events;
mod future_queue;
mod task;
mod waker;

pub use events::{EventID, EventManager};
pub use future_queue::FutureQueue;

/// Runs a local executor on `future`
pub fn run(signal_number: c_int, future: impl Future<Output = ()> + 'static) -> linux::Result<()> {
    let queue = FutureQueue::new();
    queue.push(future);
    run_queue(signal_number, queue)
}

/// Executes the tasks in the [`FutureQueue`]
pub fn run_queue(signal_number: c_int, queue: FutureQueue) -> linux::Result<()> {
    let event_manager = EventManager::new(signal_number)?;

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
