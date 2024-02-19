use super::TimerFD;
use executor::Result;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A future which yields after a certain duration
pub struct Sleep {
    /// The timer that will fire when the sleep ends
    #[allow(unused)]
    timer: TimerFD,
}

/// Sleep until `duration` has passed
pub fn sleep(duration: Duration) -> Result<Sleep> {
    Sleep::new(duration)
}

impl Sleep {
    /// Creates a new [`Sleep`] which yields after `duration` has passed
    pub fn new(duration: Duration) -> Result<Self> {
        let mut timer = TimerFD::new()?;
        timer.set(duration, None)?;

        Ok(Sleep { timer })
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("Sleep::poll()")
    }
}
