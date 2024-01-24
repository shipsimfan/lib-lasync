use super::FutureQueue;
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

/// A [`Future`] which can re-schedule itself
pub(super) struct Task {
    /// The [`Future`] to be driven to completion
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>,

    /// The queue to re-schedule the future on
    future_queue: FutureQueue,
}

impl Task {
    /// Creates a new [`Task`] for `future`
    pub(super) fn new(
        future: impl Future<Output = ()> + Send + 'static,
        future_queue: FutureQueue,
    ) -> Self {
        Task {
            future: Mutex::new(Some(Box::pin(future))),
            future_queue,
        }
    }

    pub(super) fn future(
        &self,
    ) -> &Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>> {
        &self.future
    }

    pub(super) fn wake(self: &Arc<Self>) {
        let cloned = self.clone();
        self.future_queue.push_raw(cloned);
    }
}
