use super::WaitableTimer;
use crate::{EventRef, Result};
use executor::EventManager;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A future which yields after a fixed period
pub struct Interval {
    /// The timer the fires to indicate the interval has passed
    #[allow(unused)]
    timer: WaitableTimer,

    /// The event id this is registered under
    event_id: EventRef,
}

/// A future which yields after one tick from an [`Interval`]
pub struct Tick<'a> {
    /// The interval to yield for
    interval: &'a mut Interval,
}

/// Creates an [`Interval`] future which yields immediately then yields every `period`
pub fn interval(period: Duration) -> Result<Interval> {
    interval_with_delay(Duration::ZERO, period)
}

/// Creates an [`Interval`] future which first yields after `delay` then yields every `period`
pub fn interval_with_delay(delay: Duration, period: Duration) -> Result<Interval> {
    Interval::new(delay, period)
}

impl Interval {
    /// Creates a new [`Interval`] that first yields after `delay` and then yields every `period`
    pub fn new(delay: Duration, period: Duration) -> Result<Interval> {
        let event_id = EventRef::register()?;

        let mut timer = WaitableTimer::new()?;
        timer.set(delay, Some(period), *event_id)?;

        Ok(Interval { timer, event_id })
    }

    /// Returns a future which will yield after the next timer tick
    pub fn tick(&mut self) -> Tick {
        Tick { interval: self }
    }
}

impl<'a> Future for Tick<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        EventManager::get_local_mut(|manager| {
            let event = manager.get_event_mut(*self.interval.event_id).unwrap();

            if event.get_data() > 0 {
                *event.data_mut() -= 1;
                return Poll::Ready(());
            }

            event.set_waker(Some(cx.waker().clone()));
            Poll::Pending
        })
    }
}
