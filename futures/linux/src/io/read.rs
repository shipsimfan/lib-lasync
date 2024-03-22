use std::future::Future;

use executor::Result;

/// Asynchronous equivalent of [`std::io::Read`]
pub trait Read {
    /// Attempts to read from this into `buf`, returning the number of bytes read
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize>>;
}
