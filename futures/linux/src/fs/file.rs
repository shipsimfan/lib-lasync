use executor::{platform::linux::unistd::close, Result};
use std::ffi::c_int;

use crate::{
    fd::{AsFD, FDRead, FDWrite},
    io::{Read, Write},
};

/// An open file on the filesystem
pub struct File(c_int);

impl File {
    /// Creates a new [`File`] for `fd`
    pub(super) fn new(fd: c_int) -> Self {
        File(fd)
    }
}

impl AsFD for File {
    unsafe fn fd(&self) -> c_int {
        self.0
    }
}

impl Read for File {
    fn read<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> impl std::future::Future<Output = Result<usize>> + 'a {
        FDRead::new(self, buf)
    }
}

impl Write for File {
    fn write<'a>(
        &'a mut self,
        buf: &'a [u8],
    ) -> impl std::future::Future<Output = Result<usize>> + 'a {
        FDWrite::new(self, buf)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { close(self.0) };
    }
}
