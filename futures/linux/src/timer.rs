use crate::executor::{EventID, EventManager};
use linux::{
    time::{
        itimerspec, timer_create, timer_delete, timer_gettime, timer_settime, timer_t, timespec,
    },
    try_linux,
};
use std::{
    future::Future,
    pin::Pin,
    ptr::null_mut,
    task::{Context, Poll},
    time::Duration,
};

/// A future that signals after a certain duration
pub struct Timer {
    id: EventID,

    timer: timer_t,
}

impl Timer {
    /// Creates and starts a new [`Timer`] for `duration`
    pub fn new(duration: Duration) -> linux::Result<Self> {
        // Register a new event
        let (id, mut event) = EventManager::register_signal();

        // Create the timer
        let mut timer = null_mut();
        try_linux!(timer_create(0, &mut event, &mut timer))?;

        // Start the timer
        let timerspec = itimerspec {
            value: timespec {
                sec: duration.as_secs() as _,
                nsec: duration.subsec_nanos() as _,
            },
            ..Default::default()
        };
        try_linux!(timer_settime(timer, 0, &timerspec, null_mut()))?;

        Ok(Timer { id, timer })
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Get the time remaining on the timer
        let mut timer_value = itimerspec::default();
        try_linux!(timer_gettime(self.timer, &mut timer_value)).unwrap();

        // If the timer is expired, return ready
        if timer_value.value.sec == 0 && timer_value.value.nsec == 0 {
            return Poll::Ready(());
        }

        // Register the waker if the timer isn't expired
        EventManager::set_waker(self.id, cx.waker().clone());
        Poll::Pending
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        // Unregister the event
        EventManager::unregister(self.id).unwrap();

        // Close the timer
        unsafe { timer_delete(self.timer) };
    }
}
