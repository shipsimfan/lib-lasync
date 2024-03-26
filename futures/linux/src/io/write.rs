use executor::Result;
use std::future::Future;

/// Asynchronous equivalent of [`std::io::Write`]
pub trait Write {
    /// Attempts to write the contents of `buf` into this, returning the number of bytes written
    fn write<'a>(&'a mut self, buf: &'a [u8]) -> impl Future<Output = Result<usize>> + 'a;

    /// Attempts to write all the contents of `buf` in this, returning an error if it is unable to
    fn write_all<'a>(&'a mut self, mut buf: &'a [u8]) -> impl Future<Output = Result<()>> + 'a {
        async move {
            while !buf.is_empty() {
                match self.write(buf).await {
                    Ok(0) => return Err(linux::Error::new(linux::errno::ECONNRESET)),
                    Ok(n) => buf = &buf[n..],
                    Err(error) => return Err(error),
                }
            }

            Ok(())
        }
    }
}
