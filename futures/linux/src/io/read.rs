use executor::{platform::linux::errno::ECONNRESET, Error, Result};
use std::future::Future;

/// Asynchronous equivalent of [`std::io::Read`]
pub trait Read {
    /// Attempts to read from this into `buf`, returning the number of bytes read
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> impl Future<Output = Result<usize>> + 'a;

    /// Attempts to read from this and fill `buf`, returning an error if it is unable to
    fn read_exact<'a>(
        &'a mut self,
        mut buf: &'a mut [u8],
    ) -> impl Future<Output = Result<()>> + 'a {
        async move {
            while !buf.is_empty() {
                match self.read(buf).await {
                    Ok(0) => break,
                    Ok(n) => buf = &mut buf[n..],
                    Err(error) => return Err(error),
                }
            }

            if buf.is_empty() {
                Ok(())
            } else {
                // TODO: Find a better error for this, or create a custom one
                Err(Error::new(ECONNRESET))
            }
        }
    }
}
