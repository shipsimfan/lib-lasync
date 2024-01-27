use super::FutureQueue;
use std::{cell::RefCell, future::Future, pin::Pin, rc::Rc};

/// A [`Future`] which can re-schedule itself
pub(super) struct Task {
    /// The [`Future`] to be driven to completion
    future: RefCell<Option<Pin<Box<dyn Future<Output = ()>>>>>,

    /// The queue to re-schedule the future on
    future_queue: FutureQueue,
}

impl Task {
    /// Creates a new [`Task`] for `future`
    pub(super) fn new(
        future: impl Future<Output = ()> + 'static,
        future_queue: FutureQueue,
    ) -> Self {
        Task {
            future: RefCell::new(Some(Box::pin(future))),
            future_queue,
        }
    }

    /// Gets the underyling [`Future`]
    pub(super) fn future(&self) -> &RefCell<Option<Pin<Box<dyn Future<Output = ()>>>>> {
        &self.future
    }

    /// Queues this [`Task`]'s [`Future`] to be polled by the executor
    pub(super) fn wake(self: &Rc<Self>) {
        let cloned = self.clone();
        self.future_queue.push_raw(cloned);
    }
}
