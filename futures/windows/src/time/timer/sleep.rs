use super::Timer;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A [`Future`] which yields after a period of time
pub struct TimerSleep<'a> {
    timer: &'a mut Timer,
}

impl<'a> TimerSleep<'a> {
    /// Creates a new [`TimerSleep`] future
    pub(super) fn new(timer: &'a mut Timer, duration: Duration) -> crate::Result<Self> {
        let event_id = timer.event_id();
        timer.timer().set(duration, None, event_id)?;

        Ok(TimerSleep { timer })
    }
}

impl<'a> Future for TimerSleep<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("TimerSleep::poll()")
    }
}

impl<'a> Drop for TimerSleep<'a> {
    fn drop(&mut self) {
        self.timer.cancel().ok();
    }
}
