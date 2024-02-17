use super::{Timer, TimerSleep};
use crate::Result;
use futures_common::{Select, SelectResult};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A [`Future`] which yields when either the contained [`Future`] yields or a timeout passes
pub struct TimerTimeout<'a, F: Future>(Select<F, TimerSleep<'a>>);

impl<'a, F: Future> TimerTimeout<'a, F> {
    /// Creates a new [`TimerTimeout`]
    pub(super) fn new(timer: &'a mut Timer, future: F, timeout: Duration) -> Result<Self> {
        let sleep = timer.sleep(timeout)?;
        let inner = Select::new(future, sleep);

        Ok(TimerTimeout(inner))
    }

    /// Gets the contained [`Select`]
    ///
    /// # SAFTEY
    /// This is the only way to access the contained [`Select`], do not acces it directly.
    unsafe fn project(self: Pin<&mut Self>) -> Pin<&mut Select<F, TimerSleep<'a>>> {
        self.map_unchecked_mut(|timeout| &mut timeout.0)
    }
}

impl<'a, F: Future> Future for TimerTimeout<'a, F> {
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(result) = unsafe { self.project() }.poll(cx) {
            if let SelectResult::A(value) = result {
                return Poll::Ready(Some(value));
            }

            return Poll::Ready(None);
        }

        Poll::Pending
    }
}
