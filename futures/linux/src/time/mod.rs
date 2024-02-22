//! Futures for time keeping

mod interval;
mod sleep;

pub use interval::{interval, Interval};
pub use sleep::{sleep, Sleep};
