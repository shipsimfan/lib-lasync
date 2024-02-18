use super::Timer;
use crate::time::sleep::sleep_poll;
use executor::Result;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A [`Future`] which yields after a period of time
pub struct TimerSleep<'a>(&'a mut Timer);

impl<'a> TimerSleep<'a> {
    /// Creates a new [`TimerSleep`] future
    pub(super) fn new(timer: &'a mut Timer, duration: Duration) -> Result<Self> {
        let event_id = timer.event_id();
        timer.timer().set(duration, None, event_id)?;

        Ok(TimerSleep(timer))
    }
}

impl<'a> Future for TimerSleep<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        sleep_poll(self.0.event_id(), cx)
    }
}

impl<'a> Drop for TimerSleep<'a> {
    fn drop(&mut self) {
        self.0.cancel().ok();
    }
}
