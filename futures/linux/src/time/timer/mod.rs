use executor::Result;
use std::time::Duration;

mod sleep;

pub use sleep::TimerSleep;

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
}
