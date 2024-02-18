use executor::Result;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A future which yields after a certain duration
pub struct Sleep {}

/// Sleep until `duration` has passed
pub fn sleep(duration: Duration) -> Result<Sleep> {
    todo!("sleep()");
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("Sleep::poll()")
    }
}
