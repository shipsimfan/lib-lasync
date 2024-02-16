//! Futures for time keeping

mod inner;
mod sleep;

pub use sleep::{sleep, Sleep};

use inner::WaitableTimer;