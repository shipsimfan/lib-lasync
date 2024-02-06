use std::task::Waker;

/// A queued event
pub(crate) struct Event {
    /// The [`Waker`] used to enqueue the task
    waker: Option<Waker>,
}

impl Event {
    /// Creates a new [`Event`]
    fn new() -> Self {
        Event { waker: None }
    }

    /// Queues the associated task to be run
    fn wake(&mut self) {
        match self.waker.take() {
            Some(waker) => waker.wake(),
            None => {}
        }
    }

    /// Sets the [`Waker`] associated with the event
    fn set_waker(&mut self, waker: Option<Waker>) {
        self.waker = waker;
    }
}
