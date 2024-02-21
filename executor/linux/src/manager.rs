use crate::{EventHandler, IOURing, Result, SQE};
use executor_common::{Event, EventID, List, NoPoll};
use std::{num::NonZeroUsize, ptr::null_mut};
use uring::{io_uring_cqe_get_data64, io_uring_sqe_set_data64};

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

    /// Gets an [`SQE`] for I/O submission
    pub fn get_sqe(&mut self, event_id: EventID) -> Result<SQE> {
        let sqe = self
            .io_uring
            .get_sqe()
            .ok_or(linux::Error::new(linux::errno::ENOSPC))?;

        unsafe { io_uring_sqe_set_data64(sqe, event_id.into_u64()) };

        Ok(SQE::new(sqe, &mut self.io_uring))
    }

    /// Deregisters an event based on its [`EventID`]
    pub fn deregister(&mut self, event_id: EventID) {
        self.events.remove(event_id);
    }

    /// Sleeps until an event is triggered
    pub fn poll(&mut self) -> Result<NoPoll<crate::Error>> {
        let mut cqe = null_mut();

        loop {
            self.io_uring.wait(&mut cqe)?;

            let user_data = unsafe { io_uring_cqe_get_data64(cqe) };
            let event_id = unsafe { EventID::from_u64(user_data) };

            match self.events.get_mut(event_id) {
                Some(event) => {
                    event.data_mut().run(unsafe { &mut *cqe });
                    event.wake()
                }
                None => {}
            }

            self.io_uring.seen(cqe);

            if self.io_uring.available_events() == 0 {
                return Ok(NoPoll::new());
            }
        }
    }
}
