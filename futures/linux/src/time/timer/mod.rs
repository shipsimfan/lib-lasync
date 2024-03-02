use executor::Result;
use std::{future::Future, time::Duration};

mod interval;
mod sleep;
mod timeout;

pub use interval::TimerInterval;
pub use sleep::TimerSleep;
pub use timeout::TimerTimeout;

/// A timer which can be used to make repeated time-based calls
pub struct Timer {
    /// Prevents this struct from being constructed elsewhere
    _priv: (),
}

impl Timer {
    /// Creates a new [`Timer`]
    pub fn new() -> Result<Self> {
        Ok(Timer { _priv: () })
    }

    /// Creates a [`TimerSleep`] future which yields after `duration`
    pub fn sleep(&mut self, duration: Duration) -> Result<TimerSleep> {
        TimerSleep::new(duration)
    }

    /// Creates an [`TimerInterval`] future which yields immediately then yields every `period`
    pub fn interval(&mut self, period: Duration) -> Result<TimerInterval> {
        TimerInterval::new(period)
    }

    /// Creates a [`TimerTimeout`] future which yields when either `future` yields or `timeout`
    /// passes
    pub fn timeout<F: Future>(&mut self, future: F, timeout: Duration) -> Result<TimerTimeout<F>> {
        TimerTimeout::new(self, future, timeout)
    }
}
