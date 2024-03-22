use executor::Result;
use std::future::Future;

/// Asynchronous equivalent of [`std::io::Write`]
pub trait Write {
    /// Attempts to write the contents of `buf` into this, returning the number of bytes written
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = Result<usize>>;

    /// Flushes all data that hasn't been fully written
    fn flush(&mut self) -> impl Future<Output = Result<()>>;
}
