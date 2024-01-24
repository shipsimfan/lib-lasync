//! Single-threaded executor for async/await

use events::EventManager;
use std::{future::Future, task::Context};
use task::Task;
use waker::WakerRef;

mod events;
mod future_queue;
mod task;
mod waker;

pub use future_queue::FutureQueue;

/// Runs a local executor on `future`
pub fn run(future: impl Future + 'static) -> ! {
    let queue = FutureQueue::new();
    queue.push(future);
    run_queue(queue)
}

/// Executes the tasks in the [`FutureQueue`]
pub fn run_queue<T>(queue: FutureQueue<T>) -> ! {
    let event_manager = EventManager::new();

    loop {
        // TODO: Poll event manager

        if let Some(task) = queue.pop() {
            let mut future_slot = task.future().borrow_mut();
            if let Some(mut future) = future_slot.take() {
                let waker = WakerRef::new(&task);
                let context = &mut Context::from_waker(&waker);

                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}
