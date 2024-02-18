//! Futures for time keeping

mod interval;
mod sleep;

pub use interval::{interval, interval_with_delay, Interval};
pub use sleep::{sleep, Sleep};
