//! Futures for time keeping

mod inner;
mod interval;
mod sleep;
mod timeout;
mod timer;

pub use interval::{interval, interval_with_delay, Interval};
pub use sleep::{sleep, Sleep};
pub use timeout::{timeout, Timeout};
pub use timer::{Timer, TimerInterval, TimerSleep, TimerTimeout};

use inner::WaitableTimer;
