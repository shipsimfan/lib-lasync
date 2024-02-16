use super::Sleep;
use futures_common::{Select, SelectResult};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A [`Future`] which yields when either the contained [`Future`] yields or a timeout passes
pub struct Timeout<F: Future>(Select<F, Sleep>);

/// Creates a [`Timeout`] future which yields when either `future` yields or `timeout` passes
pub fn timeout<F: Future>(future: F, timeout: Duration) -> crate::Result<Timeout<F>> {
    Timeout::new(future, timeout)
}

impl<F: Future> Timeout<F> {
    /// Creates a [`Timeout`] future which yields when either `future` yields or `timeout` passes
    pub fn new(future: F, timeout: Duration) -> crate::Result<Self> {
        let inner = Select::new(future, Sleep::new(timeout)?);

        Ok(Timeout(inner))
    }

    /// Gets the contained [`Select`]
    ///
    /// # SAFTEY
    /// This is the only way to access the contained [`Select`], do not acces it directly.
    unsafe fn project(self: Pin<&mut Self>) -> Pin<&mut Select<F, Sleep>> {
        self.map_unchecked_mut(|timeout| &mut timeout.0)
    }
}

impl<F: Future> Future for Timeout<F> {
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
