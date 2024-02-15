use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A future which yields after a certain duration
pub struct Sleep {}

/// Sleep until `duration` has passed
pub fn sleep(duration: Duration) -> win32::Result<Sleep> {
    Sleep::new(duration)
}

impl Sleep {
    /// Creates a new [`Sleep`] which yields after `duration` has passed
    pub fn new(duration: Duration) -> win32::Result<Self> {
        todo!("Sleep::new({}s)", duration.as_secs_f64())
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("Future::poll()")
    }
}
