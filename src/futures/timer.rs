use crate::executor::{EventManager, SignalValue};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A future that signals after a certain duration
pub struct Timer {
    id: Pin<Box<SignalValue>>,
}

impl Timer {
    /// Creates and starts a new [`Timer`] for `duration`
    pub fn new(duration: Duration) -> linux::Result<Self> {
        // Register a new event
        let (id, event) = EventManager::register_signal();

        // Create the timer
        todo!("Create a timer with `timer_create`");

        Ok(Timer { id })
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if the timer is expired
        todo!("Check if the timer is expired")
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        // Close the timer
        todo!("Close the timer");

        // Unregister the event
        EventManager::unregister(self.id.id())
    }
}
