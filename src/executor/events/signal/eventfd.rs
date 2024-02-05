use linux::{
    sys::eventfd::{eventfd, EFD_NONBLOCK},
    try_linux,
    unistd::{close, read, write},
};
use std::ffi::c_int;

/// An file descriptor for signalling events
pub(super) struct EventFD(c_int);

impl EventFD {
    /// Creates a new [`EventFD`]
    pub(super) fn new() -> linux::Result<Self> {
        try_linux!(eventfd(0, EFD_NONBLOCK)).map(|fd| EventFD(fd))
    }

    /// Gets the underlying file descriptor
    pub(super) fn fd(&self) -> c_int {
        self.0
    }

    /// Reads the eventfd to reset the count
    pub(super) fn read(&mut self) {
        let mut buffer = [0; 8];
        unsafe { read(self.0, buffer.as_mut_ptr().cast(), 8) };
    }

    pub(super) fn signal_raw(fd: c_int) {
        let buffer = 1u64.to_le_bytes();
        unsafe { write(fd, buffer.as_ptr().cast(), 8) };
    }
}

impl Drop for EventFD {
    fn drop(&mut self) {
        unsafe { close(self.0) };
    }
}
