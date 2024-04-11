use crate::fs::Open;
use linux::fcntl::{O_APPEND, O_CREAT, O_EXCL, O_RDONLY, O_TRUNC, O_WRONLY};
use std::{ffi::c_int, path::Path};

/// Options dictating access to a file
pub struct OpenOptions(c_int);

impl OpenOptions {
    /// Creates a new [`OpenOptions`] with all options set to false
    pub const fn new() -> Self {
        OpenOptions(0)
    }

    /// Opens the file at `path` with the options specified in `self`
    pub fn open<P: AsRef<Path>>(&self, path: P) -> Open {
        Open::new(path.as_ref(), self.0)
    }

    /// Sets the read access for the file
    pub fn read(&mut self, read: bool) -> &mut Self {
        self.set(O_RDONLY, read);
        self
    }

    /// Sets the write access for the file
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.set(O_WRONLY, write);
        self
    }

    /// Sets if writes will be automatically appended to the end of the file.
    ///
    /// See [`std::fs::OpenOptions::append`] for more details.
    pub fn append(&mut self, append: bool) -> &mut Self {
        self.set(O_APPEND, append);
        if append {
            self.set_flag(O_WRONLY);
        }
        self
    }

    /// Sets if the file will be truncated to zero length upon opening
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.set(O_TRUNC, truncate);
        self
    }

    /// Sets if the file should be created if it does not already exist. If the file already
    /// exists, it will be opened normally.
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.set(O_CREAT, create);
        self
    }

    /// Sets if the file should be created if it does not already exist. If the file already
    /// exists, the open will fail with an error.
    pub fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.set(O_EXCL, create_new);
        self.set(O_CREAT, create_new);
        self
    }

    /// Sets `flag` to `value`
    fn set(&mut self, flag: c_int, value: bool) {
        if value {
            self.set_flag(flag)
        } else {
            self.clear_flag(flag)
        }
    }

    /// Sets `flag` in the options
    fn set_flag(&mut self, flag: c_int) {
        self.0 |= flag;
    }

    /// Clears `flag` in the options
    fn clear_flag(&mut self, flag: c_int) {
        self.0 &= !flag;
    }
}
