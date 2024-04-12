use linux::unistd::close;
use std::ffi::c_int;

/// An open file on the filesystem
pub struct File(c_int);

impl File {
    /// Creates a new [`File`] for `fd`
    pub(super) fn new(fd: c_int) -> Self {
        File(fd)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { close(self.0) };
    }
}
