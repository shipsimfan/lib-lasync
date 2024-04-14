use crate::{Error, Result};
use std::ptr::null_mut;
use uring::{
    io_uring, io_uring_cq_ready, io_uring_cqe, io_uring_cqe_seen, io_uring_get_sqe,
    io_uring_queue_exit, io_uring_queue_init, io_uring_sqe, io_uring_submit, io_uring_wait_cqe,
};

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

    /// Attempts to get an [`io_uring_sqe`] from the ring
    pub(crate) fn get_sqe(&mut self) -> Option<*mut io_uring_sqe> {
        let result = unsafe { io_uring_get_sqe(&mut self.inner) };
        if result == null_mut() {
            None
        } else {
            Some(result.cast())
        }
    }

    /// Submits an [`io_uring_sqe`] to poll for completion
    #[allow(unused_variables)]
    pub(crate) fn submit_sqe(&mut self, sqe: *mut io_uring_sqe) -> Result<()> {
        let result = unsafe { io_uring_submit(&mut self.inner) };
        if result < 0 {
            Err(Error::new(-result))
        } else {
            Ok(())
        }
    }

    /// Waits until an event is triggered, then puts the event into the wait queue
    pub(crate) fn wait(&mut self, cqe: &mut *mut io_uring_cqe) -> Result<()> {
        let result = unsafe { io_uring_wait_cqe(&mut self.inner, cqe) };
        if result < 0 {
            Err(Error::new(-result))
        } else {
            Ok(())
        }
    }

    /// Marks an [`io_uring_cqe`] as seen
    pub(crate) fn seen(&mut self, cqe: *mut io_uring_cqe) {
        unsafe { io_uring_cqe_seen(&mut self.inner, cqe) }
    }

    /// Gets the number of events that have already been triggered
    pub(crate) fn available_events(&self) -> u32 {
        unsafe { io_uring_cq_ready(&self.inner) }
    }
}

impl Drop for IOURing {
    fn drop(&mut self) {
        unsafe { io_uring_queue_exit(&mut self.inner) };
    }
}

impl !Send for IOURing {}
impl !Sync for IOURing {}
