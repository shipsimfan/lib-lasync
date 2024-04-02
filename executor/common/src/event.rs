use std::task::Waker;

/// A queued event
pub struct Event<T = ()> {
    /// The [`Waker`] used to enqueue the task
    waker: Option<Waker>,

    /// Optional data the platform can associate with an event
    data: T,
}

impl<T> Event<T> {
    /// Creates a new [`Event`]
    pub fn new(data: T) -> Self {
        Event { waker: None, data }
    }

    /// Gets the data associated with an event
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Mutably gets the data associated with an event
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Sets the data associated with an event, returning the old value
    pub fn set_data(&mut self, mut data: T) -> T {
        std::mem::swap(&mut self.data, &mut data);
        data
    }

    /// Queues the associated task to be run
    pub fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    /// Sets the [`Waker`] associated with the event
    pub fn set_waker(&mut self, waker: Option<Waker>) {
        self.waker = waker;
    }
}

impl<T: Copy> Event<T> {
    /// Gets the data associated with an event
    pub fn get_data(&self) -> T {
        self.data
    }
}

impl<T> !Send for Event<T> {}
impl<T> !Sync for Event<T> {}
