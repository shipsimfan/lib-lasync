use super::WaitableTimer;
use crate::EventRef;
use executor::{EventID, EventManager};
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
pub fn interval(period: Duration) -> crate::Result<Interval> {
    interval_with_delay(Duration::ZERO, period)
}

/// Creates an [`Interval`] future which first yields after `delay` then yields every `period`
pub fn interval_with_delay(delay: Duration, period: Duration) -> crate::Result<Interval> {
    Interval::new(delay, period)
}

/// Creates and starts a timer to a given duration
fn create_and_set_timer(
    delay: Duration,
    period: Duration,
    event_id: EventID,
) -> crate::Result<WaitableTimer> {
    let mut timer = WaitableTimer::new()?;
    timer.set(delay, Some(period), event_id)?;
    Ok(timer)
}

/// Attempts to register an event with the local event manager
fn register() -> crate::Result<EventID> {
    EventManager::get_local_mut(|manager| manager.register(0))
        .ok_or(win32::Error::new(win32::ERROR_TOO_MANY_CMDS))
}

impl Interval {
    /// Creates a new [`Interval`] that first yields after `delay` and then yields every `period`
    pub fn new(delay: Duration, period: Duration) -> crate::Result<Interval> {
        let event_id = EventRef::new(register()?);

        let timer = create_and_set_timer(delay, period, *event_id)?;

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
