use crate::{Error, Result};
use linux::{
    fcntl::O_RDONLY,
    stdlib::free,
    sys::stat::open,
    try_linux,
    unistd::{close, getcwd},
};
use std::{ffi::c_int, ptr::null_mut};

/// A descriptor to the current working directory
pub(crate) struct WorkingDirectory(c_int);

impl WorkingDirectory {
    /// Opens a file descriptor to the current working directory
    pub(crate) fn open() -> Result<Self> {
        let path = unsafe { getcwd(null_mut(), 0) };
        if path == null_mut() {
            return Err(Error::errno());
        }

        let result = try_linux!(open(path, O_RDONLY)).map(|fd| WorkingDirectory(fd));
        unsafe { free(path as _) };
        result
    }

    /// Gets the underlying file descriptor
    ///
    /// # Safety
    /// The caller must not close the descriptor and must use it appropriately for a directory
    /// descriptor.
    pub(crate) unsafe fn inner(&self) -> c_int {
        self.0
    }
}

impl Drop for WorkingDirectory {
    fn drop(&mut self) {
        unsafe { close(self.0) };
    }
}
