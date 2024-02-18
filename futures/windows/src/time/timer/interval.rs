use super::Timer;
use crate::time::interval::interval_poll;
use executor::Result;
use std::{future::Future, time::Duration};

/// A future which yields after a fixed period
pub struct TimerInterval<'a>(&'a mut Timer);

/// A future which yields after one tick from [`TimerInterval`]
pub struct TimerTick<'a, 'b: 'a>(&'a mut TimerInterval<'b>);

impl<'a> TimerInterval<'a> {
    /// Creates a new [`TimerInterval`]
    pub(super) fn new(timer: &'a mut Timer, delay: Duration, period: Duration) -> Result<Self> {
        let event_id = timer.event_id();
        timer.timer().set(delay, Some(period), event_id)?;

        Ok(TimerInterval(timer))
    }

    /// Returns a future which will yield after the next timer tick
    pub fn tick<'b>(&'b mut self) -> TimerTick<'b, 'a> {
        TimerTick(self)
    }
}

impl<'a, 'b> Future for TimerTick<'a, 'b> {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        interval_poll(self.0 .0.event_id(), cx)
    }
}
