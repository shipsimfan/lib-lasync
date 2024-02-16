//! Futures for time keeping

mod inner;
mod interval;
mod sleep;
mod timeout;

pub use interval::{interval, interval_with_delay, Interval};
pub use sleep::{sleep, Sleep};
pub use timeout::{timeout, Timeout};

use inner::WaitableTimer;
