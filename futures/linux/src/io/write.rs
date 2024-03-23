use executor::Result;
use std::future::Future;

/// Asynchronous equivalent of [`std::io::Write`]
pub trait Write {
    /// Attempts to write the contents of `buf` into this, returning the number of bytes written
    fn write<'a>(&'a mut self, buf: &'a [u8]) -> impl Future<Output = Result<usize>> + 'a;
}
