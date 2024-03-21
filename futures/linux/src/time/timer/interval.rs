use crate::time::{interval::Tick, Interval};
use executor::Result;
use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A future which yields after a fixed period
pub struct TimerInterval<'a> {
    /// The actual interval future
    inner: Interval,

    /// A marker for the lifetime
    _lifetime: PhantomData<&'a mut ()>,
}

/// A future which yields after one tick from [`TimerInterval`]
pub struct TimerTick<'a, 'b: 'a> {
    /// The actual tick future
    inner: Tick<'a>,

    /// A marker for the lifetime
    _lifetime: PhantomData<&'b mut ()>,
}

impl<'a> TimerInterval<'a> {
    /// Creates a new [`TimerInterval`] future
    pub(super) fn new(period: Duration) -> Result<Self> {
        Ok(TimerInterval {
            inner: Interval::new(period)?,
            _lifetime: PhantomData,
        })
    }

    /// Returns a future which will yield after the next timer tick
    pub fn tick<'b>(&'b mut self) -> TimerTick<'b, 'a> {
        TimerTick {
            inner: self.inner.tick(),
            _lifetime: PhantomData,
        }
    }
}

impl<'a, 'b: 'a> Future for TimerTick<'a, 'b> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx)
    }
}
