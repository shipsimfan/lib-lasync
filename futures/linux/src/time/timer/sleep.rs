use crate::time::Sleep;
use executor::Result;
use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A [`Future`] which yields after a period of time
pub struct TimerSleep<'a> {
    /// The actual sleep future
    inner: Sleep,

    /// A marker for the lifetime
    _lifetime: PhantomData<&'a mut ()>,
}

impl<'a> TimerSleep<'a> {
    /// Creates a new [`TimerSleep`] future
    pub(super) fn new(duration: Duration) -> Result<Self> {
        Ok(TimerSleep {
            inner: Sleep::new(duration)?,
            _lifetime: PhantomData,
        })
    }
}

impl<'a> Future for TimerSleep<'a> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx)
    }
}

impl<'a> !Send for TimerSleep<'a> {}
impl<'a> !Sync for TimerSleep<'a> {}
