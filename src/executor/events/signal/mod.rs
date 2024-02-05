use super::EventID;
use eventfd::EventFD;
use handler::SignalHandler;
use std::{
    ffi::c_int,
    pin::Pin,
    sync::mpsc::{self, Receiver},
};

mod eventfd;
mod handler;

/// A handler for signalled events
pub(super) struct Signal {
    /// The signal handler, including [`Sender`] and `event_fd` for signalling events
    _handler: Pin<Box<SignalHandler>>,

    /// The file descriptor for signalling ready events
    event_fd: EventFD,

    /// A list of events which have been signalled
    queue: Receiver<u64>,

    /// The signal this is registered on
    signal_number: c_int,
}

impl Signal {
    pub(super) fn new(signal_number: c_int) -> linux::Result<Self> {
        let event_fd = EventFD::new()?;
        let (sender, queue) = mpsc::channel();

        let _handler = SignalHandler::new(signal_number, &event_fd, sender)?;

        Ok(Signal {
            event_fd,
            _handler,
            queue,
            signal_number,
        })
    }

    /// Gets the signal number this is registered on
    pub(super) fn signal_number(&self) -> c_int {
        self.signal_number
    }

    // Gets the file decriptor which signals when a [`EPOLLIN`] event when it is ready
    pub(super) fn fd(&self) -> c_int {
        self.event_fd.fd()
    }

    /// Gets a signalled event
    pub(super) fn read(&mut self) -> Option<EventID> {
        self.event_fd.read();

        self.queue
            .try_recv()
            .map(|event_id| EventID::from_u64(event_id))
            .ok()
    }
}
