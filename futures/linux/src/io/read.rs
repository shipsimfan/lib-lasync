use std::future::Future;

use executor::Result;

/// Asynchronous equivalent of [`std::io::Read`]
pub trait Read {
    /// Attempts to read from this into `buf`, returning the number of bytes read
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> impl Future<Output = Result<usize>> + 'a;
}
