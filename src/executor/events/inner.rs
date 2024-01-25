use super::Event;
use linux::{
    sys::epoll::{
        epoll_create, epoll_ctl, epoll_data_t, epoll_event, EPOLL_CTL_ADD, EPOLL_CTL_DEL,
        EPOLL_CTL_MOD,
    },
    try_linux,
    unistd::close,
};
use std::{ffi::c_int, ptr::null, task::Waker};

/// A set of [`Event`]s to wait on
pub(super) struct EventManagerInner {
    /// The events we are waiting on
    events: Vec<Event>,

    /// The `epoll` object to wait with
    epoll: c_int,
}

impl EventManagerInner {
    /// Creates a new [`EventManagerInner`]
    pub(super) fn new() -> linux::Result<Self> {
        let epoll = try_linux!(epoll_create(1))?;
        let events = Vec::new();

        Ok(EventManagerInner { events, epoll })
    }

    /// Gets the number of events currently being polled
    pub(super) fn count(&self) -> usize {
        self.events.len()
    }

    /// Blocks until an event becomes ready and wakes the ready events
    pub(super) fn poll(&self) -> linux::Result<()> {
        todo!()
    }

    /// (Re)registers an event to be polled
    pub(super) fn register(
        &mut self,
        file_descriptor: c_int,
        events: u32,
        waker: Waker,
    ) -> linux::Result<()> {
        let epoll = self.epoll;

        let event = self.get_event_mut(file_descriptor);

        let op = match event {
            Some(_) => EPOLL_CTL_MOD,
            None => EPOLL_CTL_ADD,
        };
        let epoll_event = epoll_event {
            events,
            data: epoll_data_t {
                fd: file_descriptor,
            },
        };
        try_linux!(epoll_ctl(epoll, op, file_descriptor, &epoll_event))?;

        match event {
            Some(event) => event.set_waker(waker),
            None => self.events.push(Event::new(file_descriptor, waker)),
        }

        Ok(())
    }

    /// Stops polling for an event
    pub(super) fn unregister(&mut self, file_descriptor: c_int) -> linux::Result<()> {
        try_linux!(epoll_ctl(
            self.epoll,
            EPOLL_CTL_DEL,
            file_descriptor,
            null()
        ))?;

        self.remove_event(file_descriptor);

        Ok(())
    }

    /// Gets an [`Event`] using its file_descriptor
    fn get_event_mut(&mut self, file_descriptor: c_int) -> Option<&mut Event> {
        for event in &mut self.events {
            if event.file_descriptor() == file_descriptor {
                return Some(event);
            }
        }

        None
    }

    /// Removes an [`Event`] using its file descriptor
    fn remove_event(&mut self, file_descriptor: c_int) {
        for i in 0..self.events.len() {
            if self.events[i].file_descriptor() == file_descriptor {
                self.events.swap_remove(i);
                return;
            }
        }
    }
}

impl Drop for EventManagerInner {
    fn drop(&mut self) {
        unsafe { close(self.epoll) };
    }
}
