use std::future::Future;

/// An asynchronous iterator over a series of elements
pub trait Iterator {
    /// The elements being iterated over
    type Item;

    /// Advances the iterator and returns the next value
    fn next(&mut self) -> impl Future<Output = Option<Self::Item>>;
}

/// An wrapper which implements asynchronous [`Iterator`] for types implementing the synchronous
/// [`std::iter::Iterator`].
pub struct SyncIter<I: std::iter::Iterator>(I);

impl<I: std::iter::Iterator> SyncIter<I> {
    /// Creates a new asynchronous wrapper over `iter`
    pub const fn new(iter: I) -> Self {
        SyncIter(iter)
    }
}

impl<I: std::iter::Iterator> Iterator for SyncIter<I> {
    type Item = I::Item;

    async fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
