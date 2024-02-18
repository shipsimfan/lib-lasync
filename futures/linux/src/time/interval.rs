use executor::Result;
use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A future which yields after a fixed period
pub struct Interval;

/// A future which yields after one tick from an [`Interval`]
pub struct Tick<'a>(PhantomData<&'a ()>);

/// Creates an [`Interval`] future which yields immediately then yields every `period`
pub fn interval(period: Duration) -> Result<Interval> {
    todo!("interval()")
}

/// Creates an [`Interval`] future which first yields after `delay` then yields every `period`
pub fn interval_with_delay(delay: Duration, period: Duration) -> Result<Interval> {
    todo!("interval_with_delay")
}

impl Interval {
    /// Returns a future which will yield after the next timer tick
    pub fn tick(&mut self) -> Tick {
        todo!("Interval::tick")
    }
}

impl<'a> Future for Tick<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("Tick::poll()")
    }
}
