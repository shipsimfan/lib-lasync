use crate::{
    fd::{AsFD, FDRead, FDWrite},
    fs::{FileStat, Open, OpenOptions},
    io::{Read, Write},
};
use executor::{platform::linux::unistd::close, Result};
use std::{ffi::c_int, path::Path};

/// An open file on the filesystem
pub struct File(c_int);

impl File {
    /// Creates a new [`File`] for `fd`
    pub(super) fn new(fd: c_int) -> Self {
        File(fd)
    }

    /// Creates a [`File`] at `path` and opens it with write permissions
    pub fn create<P: AsRef<Path>>(path: P) -> Open {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
    }

    /// Creates a new [`File`] at `path` and opens it with read and write permissions. This
    /// function fails if the file exists already.
    pub fn create_new<P: AsRef<Path>>(path: P) -> Open {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path)
    }

    /// Opens the [`File`] at `path` with read permissions
    pub fn open<P: AsRef<Path>>(path: P) -> Open {
        OpenOptions::new().read(true).open(path)
    }

    /// Returns a new [`OpenOptions`] object for opening [`File`]s
    pub fn options() -> OpenOptions {
        OpenOptions::new()
    }

    /// Returns a [`Future`] which gets the metadata about a file
    pub fn metadata(&self) -> FileStat {
        FileStat::new(self)
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
