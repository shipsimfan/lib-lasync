use crate::{Error, Objects, Result, WaitResult};
use executor_common::{Event, EventID, List, Pollable};
use std::{cell::RefCell, num::NonZeroUsize, rc::Rc};
use win32::{
    winsock2::{WSACleanup, WSAStartup, WSADATA},
    SleepEx, INFINITE, TRUE,
};

/// The manager of events on a thread
pub struct LocalEventManager {
    events: List<Event<usize>>,
    objects: Rc<RefCell<Objects>>,
}

/// Allows calling [`SleepEx`] while not holding the reference to the [`LocalEventManager`].
///
/// This is required because the APCs for events run during the call to [`SleepEx`] and directly
/// wake the events. If the [`LocalEventManager`] was held, the program would panic.
pub struct SleepPoll(Rc<RefCell<Objects>>);

impl LocalEventManager {
    /// Creates a new [`LocalEventManager`] with space for at most `size` simultaneous events
    pub fn new(size: NonZeroUsize) -> Result<Self> {
        let mut wsa_data = WSADATA::default();
        let result = unsafe { WSAStartup((2 << 8) | 2, &mut wsa_data) };
        if result != 0 {
            return Err(Error::new_win32(result as _));
        }

        let events = List::new(size);
        let objects = Rc::new(RefCell::new(Objects::new()));

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

    /// Sleeps until an event is triggered
    ///
    /// This function returns a [`SleepPoll`] because the event manager's [`RefCell`] cannot be
    /// held. This is because the individual APCs will call the wakers through this event manager
    /// during the poll.
    pub fn poll(&mut self) -> Result<SleepPoll> {
        Ok(SleepPoll(self.objects.clone()))
    }
}

impl Drop for LocalEventManager {
    fn drop(&mut self) {
        unsafe { WSACleanup() };
    }
}

impl Pollable for SleepPoll {
    type Error = crate::Error;

    fn poll(&self) -> Result<()> {
        let mut objects = self.0.borrow_mut();

        if objects.count() == 0 {
            unsafe { SleepEx(INFINITE, TRUE) };
            return Ok(());
        }

        let mut timeout = INFINITE;
        while objects.count() > 0 {
            let event_id = match objects.wait(timeout)? {
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

            todo!("Somehow wake the event...");
        }

        Ok(())
    }
}
