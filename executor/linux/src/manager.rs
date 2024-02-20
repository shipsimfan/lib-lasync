use crate::{event_handler::EventHandler, IOURing, Result};
use executor_common::{Event, EventID, List};
use std::num::NonZeroUsize;
use uring::{io_uring_sqe, io_uring_sqe_set_data64};

/// The manager of events on a thread
pub struct LocalEventManager {
    events: List<Event<EventHandler>>,

    io_uring: IOURing,
}

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`] with space for at most `size` simultaneous events
    ///
    /// # Panic
    /// This function will panic if `size` is over 8192
    pub fn new(size: NonZeroUsize) -> Result<Self> {
        assert!(size.get() <= 8192);

        let events = List::new(size);

        let io_uring = IOURing::new((size.get() / 2) as _)?;

        Ok(LocalEventManager { events, io_uring })
    }

    /// Gets the number of outstanding events
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Mutably gets an event
    pub fn get_event_mut(&mut self, event_id: EventID) -> Option<&mut Event<EventHandler>> {
        self.events.get_mut(event_id)
    }

    /// Registers a new [`EventHandler`] and allocates an [`EventID`] for it
    pub fn register(&mut self, handler: EventHandler) -> Option<EventID> {
        self.events.insert(Event::new(handler))
    }

    /// Gets an [`io_uring_sqe`] for I/O submission
    pub fn get_sqe(&mut self) -> Result<&mut io_uring_sqe> {
        self.io_uring
            .get_sqe()
            .ok_or(linux::Error::new(linux::errno::ENOSPC))
    }

    /// Submits an [`io_uring_sqe`] to be polled for completion
    pub fn submit_sqe(&mut self, sqe: &mut io_uring_sqe, event_id: EventID) -> Result<()> {
        unsafe { io_uring_sqe_set_data64(sqe, event_id.into_u64()) };

        self.io_uring.submit_sqe(sqe)
    }

    /// Sleeps until an event is triggered
    pub fn poll(&mut self) {
        todo!("poll");
    }
}
