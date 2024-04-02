use crate::{IOURing, Result};
use uring::io_uring_sqe;

/// A wrapper for [`io_uring_sqe`] that allows easy submission
pub struct SQE<'a> {
    inner: *mut io_uring_sqe,
    ring: &'a mut IOURing,
}

impl<'a> SQE<'a> {
    /// Creates a new [`SQE`] from `inner`
    pub(crate) fn new(inner: *mut io_uring_sqe, ring: &'a mut IOURing) -> Self {
        SQE { inner, ring }
    }

    /// Gets the pointer to the underlying [`io_uring_sqe`]
    pub fn as_ptr(&self) -> *mut io_uring_sqe {
        self.inner as _
    }

    /// Submits the [`SQE`] to be polled for completion
    pub fn submit(self) -> Result<()> {
        self.ring.submit_sqe(self.inner)
    }
}

impl<'a> !Send for SQE<'a> {}
impl<'a> !Sync for SQE<'a> {}
