use super::WaitableTimer;
use crate::EventRef;
use executor::{EventID, EventManager, Result};
use std::{future::Future, time::Duration};

mod interval;
mod sleep;
mod timeout;

pub use interval::TimerInterval;
pub use sleep::TimerSleep;
pub use timeout::TimerTimeout;

/// A timer which can be used to make repeated time-based calls
pub struct Timer {
    /// The underlying timer
    timer: WaitableTimer,

    /// The event id this timer has reserved
    event_id: EventRef,
}

impl Timer {
    /// Creates a new [`Timer`]
    pub fn new() -> Result<Self> {
        let event_id = EventRef::register()?;
        let timer = WaitableTimer::new()?;

        Ok(Timer { timer, event_id })
    }

    /// Creates a [`TimerSleep`] future which yields after `duration`
    pub fn sleep(&mut self, duration: Duration) -> Result<TimerSleep> {
        TimerSleep::new(self, duration)
    }

    /// Creates an [`TimerInterval`] future which yields immediately then yields every `period`
    pub fn interval(&mut self, period: Duration) -> Result<TimerInterval> {
        TimerInterval::new(self, period)
    }

    /// Creates a [`TimerTimeout`] future which yields when either `future` yields or `timeout`
    /// passes
    pub fn timeout<F: Future>(&mut self, future: F, timeout: Duration) -> Result<TimerTimeout<F>> {
        TimerTimeout::new(self, future, timeout)
    }

    /// Gets the [`EventID`] this timer has reserved
    pub(self) fn event_id(&self) -> EventID {
        *self.event_id
    }

    /// Gets the underlying timer
    pub(self) fn timer(&mut self) -> &mut WaitableTimer {
        &mut self.timer
    }

    /// Cancels any outstanding timer events
    pub(self) fn cancel(&mut self) -> Result<()> {
        EventManager::get_local_mut(|manager| {
            manager.get_event_mut(*self.event_id).unwrap().set_data(0)
        });

        self.timer.cancel()
    }
}
