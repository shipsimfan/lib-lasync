use crate::Result;
use linux::Error;
use uring::{io_uring, io_uring_queue_exit, io_uring_queue_init};

/// `io_uring` submission and completion queues
pub(crate) struct IOURing {
    inner: io_uring,
}

impl IOURing {
    /// Creates a new [`IOURing`] with `entries` entries
    ///
    /// # Panic
    /// This function will panic if `entries` is more than 4096
    pub(crate) fn new(entries: u32) -> Result<Self> {
        assert!(entries <= 4096);

        let mut inner = io_uring::default();
        let result = unsafe { io_uring_queue_init(entries, &mut inner, 0) };
        if result < 0 {
            return Err(Error::new(-result));
        }

        Ok(IOURing { inner })
    }
}

impl Drop for IOURing {
    fn drop(&mut self) {
        unsafe { io_uring_queue_exit(&mut self.inner) };
    }
}
