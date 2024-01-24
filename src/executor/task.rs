use super::FutureQueue;
use std::{cell::RefCell, future::Future, pin::Pin, rc::Rc};

/// A [`Future`] which can re-schedule itself
pub(super) struct Task<T> {
    /// The [`Future`] to be driven to completion
    future: RefCell<Option<Pin<Box<dyn Future<Output = T>>>>>,

    /// The queue to re-schedule the future on
    future_queue: FutureQueue<T>,
}

impl<T> Task<T> {
    /// Creates a new [`Task`] for `future`
    pub(super) fn new(
        future: impl Future<Output = T> + 'static,
        future_queue: FutureQueue<T>,
    ) -> Self {
        Task {
            future: RefCell::new(Some(Box::pin(future))),
            future_queue,
        }
    }

    pub(super) fn future(&self) -> &RefCell<Option<Pin<Box<dyn Future<Output = T>>>>> {
        &self.future
    }

    pub(super) fn wake(self: &Rc<Self>) {
        let cloned = self.clone();
        self.future_queue.push_raw(cloned);
    }
}
