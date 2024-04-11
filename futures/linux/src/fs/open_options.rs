use crate::fs::Open;
use executor::{Error, Result};
use linux::{
    errno::EINVAL,
    fcntl::{O_APPEND, O_CREAT, O_EXCL, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY},
};
use std::{ffi::c_int, path::Path};

/// Options dictating access to a file
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

impl OpenOptions {
    /// Creates a new [`OpenOptions`] with all options set to false
    pub const fn new() -> Self {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
        }
    }

    /// Opens the file at `path` with the options specified in `self`
    pub fn open<P: AsRef<Path>>(&self, path: P) -> Open {
        Open::new(path.as_ref(), self.get_options())
    }

    /// Sets the read access for the file
    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }

    /// Sets the write access for the file
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    /// Sets if writes will be automatically appended to the end of the file.
    ///
    /// See [`std::fs::OpenOptions::append`] for more details.
    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        self
    }

    /// Sets if the file will be truncated to zero length upon opening
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    /// Sets if the file should be created if it does not already exist. If the file already
    /// exists, it will be opened normally.
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }

    /// Sets if the file should be created if it does not already exist. If the file already
    /// exists, the open will fail with an error.
    pub fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.create_new = create_new;
        self
    }

    /// Calculates the options [`c_int`] for `self`
    fn get_options(&self) -> Result<c_int> {
        let mut options = self.get_access()?;

        if self.append {
            options |= O_APPEND;
        }

        if self.truncate {
            options |= O_TRUNC;
        }

        if self.create_new {
            if self.create || self.truncate {
                return Err(Error::new(EINVAL));
            }

            options |= O_EXCL;
            options |= O_CREAT;
        }

        if self.create {
            options |= O_CREAT;
        }

        if (self.truncate || self.create || self.create_new) && !(self.write || self.append) {
            return Err(Error::new(EINVAL));
        }

        Ok(options)
    }

    /// Gets the access level for the file
    fn get_access(&self) -> Result<c_int> {
        let write = self.write || self.append;

        match (self.read, write) {
            (false, false) => Err(Error::new(EINVAL)),
            (true, false) => Ok(O_RDONLY),
            (false, true) => Ok(O_WRONLY),
            (true, true) => Ok(O_RDWR),
        }
    }
}
