use crate::FutureQueue;
use std::{cell::RefCell, future::Future, pin::Pin, rc::Rc};

/// A [`Future`] which can re-schedule itself
pub(crate) struct Task<'a> {
    /// The [`Future`] to be driven to completion
    future: RefCell<Option<Pin<Box<dyn Future<Output = ()> + 'a>>>>,

    /// The queue to re-schedule the future on
    future_queue: FutureQueue<'a>,
}

impl<'a> Task<'a> {
    /// Creates a new [`Task`] for `future`
    pub(super) fn new(
        future: impl Future<Output = ()> + 'a,
        future_queue: FutureQueue<'a>,
    ) -> Self {
        Task {
            future: RefCell::new(Some(Box::pin(future))),
            future_queue,
        }
    }

    /// Gets the underyling [`Future`]
    pub(crate) fn future(&self) -> &RefCell<Option<Pin<Box<dyn Future<Output = ()> + 'a>>>> {
        &self.future
    }

    /// Queues this [`Task`]'s [`Future`] to be polled by the executor
    pub(super) fn wake(self: &Rc<Self>) {
        let cloned = self.clone();
        self.future_queue.push_raw(cloned);
    }
}

impl<'a> !Send for Task<'a> {}
impl<'a> !Sync for Task<'a> {}
