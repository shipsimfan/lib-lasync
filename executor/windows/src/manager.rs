use crate::{Error, Objects, Result, WaitResult};
use executor_common::{Event, EventID, List};
use std::num::NonZeroUsize;
use win32::{
    winsock2::{WSACleanup, WSAStartup, WSADATA},
    SleepEx, INFINITE, TRUE,
};

/// The manager of events on a thread
pub struct LocalEventManager {
    events: List<Event<usize>>,
    objects: Objects,
}

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`] with space for at most `size` simultaneous events
    pub fn new(size: NonZeroUsize) -> Result<Self> {
        let mut wsa_data = WSADATA::default();
        let result = unsafe { WSAStartup((2 << 8) | 2, &mut wsa_data) };
        if result != 0 {
            return Err(Error::new_win32(result as _));
        }

        let events = List::new(size);
        let objects = Objects::new();

        Ok(LocalEventManager { events, objects })
    }

    /// Gets the number of outstanding events
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Gets an event
    pub fn get_event(&self, event_id: EventID) -> Option<&Event<usize>> {
        self.events.get(event_id)
    }

    /// Registers a new event, returning the [`EventID`] if there is enough room
    pub fn register(&mut self, initial_value: usize) -> Option<EventID> {
        self.events.insert(Event::new(initial_value))
    }

    /// Mutably gets an event
    pub fn get_event_mut(&mut self, event_id: EventID) -> Option<&mut Event<usize>> {
        self.events.get_mut(event_id)
    }

    /// Deregisters an event based on its [`EventID`]
    pub fn deregister(&mut self, event_id: EventID) {
        self.events.remove(event_id);
    }

    /// Sleeps until an event is triggered and wake all triggered events
    pub fn poll(&mut self) -> Result<()> {
        if self.objects.count() == 0 {
            unsafe { SleepEx(INFINITE, TRUE) };
            return Ok(());
        }

        let mut timeout = INFINITE;
        while self.objects.count() > 0 {
            let event_id = match self.objects.wait(timeout)? {
                WaitResult::Timeout => break,
                WaitResult::IOCompletion => {
                    timeout = 0;
                    continue;
                }
                WaitResult::Object(event_id) => {
                    timeout = 0;
                    event_id
                }
            };

            self.events.get_mut(event_id).map(|event| event.wake());
        }

        Ok(())
    }
}

impl Drop for LocalEventManager {
    fn drop(&mut self) {
        unsafe { WSACleanup() };
    }
}
