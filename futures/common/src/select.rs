use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A [`Future`] which yields when one of the two contained [`Future`]s yield
pub struct Select<A: Future, B: Future> {
    a: A,
    b: B,
}

/// The result return from a [`Select`] future
pub enum SelectResult<A, B> {
    /// The future in `a` yielded first
    A(A),

    /// The future in `b` yielded first
    B(B),
}

/// Creates a [`Select`] future which yields when either of the contained [`Future`]s yield
pub fn select<A: Future, B: Future>(a: A, b: B) -> Select<A, B> {
    Select::new(a, b)
}

/// Creates a [`Select`] future which yields when any of the contained [`Future`]s yield
#[macro_export]
macro_rules! select {
    ($a: expr, $b: expr) => {
        $crate::select($a, $b)
    };

    ($a: expr, $b: expr, $($remaining: expr),+) => {
        $crate::select!($crate::select($a, $b), $($remaining),+)
    }
}

impl<A: Future, B: Future> Select<A, B> {
    /// Creates a new [`Select`] future
    pub fn new(a: A, b: B) -> Self {
        todo!("Select::new()")
    }
}

impl<A: Future, B: Future> Future for Select<A, B> {
    type Output = SelectResult<A::Output, B::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("Select::poll()")
    }
}
