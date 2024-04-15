use executor::platform::linux::sys::stat::{s_isdir, s_islnk, Statx};

use crate::fs::FileType;

/// Metadata about a file or directory
pub struct Metadata {
    /// The type of the file
    file_type: FileType,

    /// The length of the file in bytes
    length: u64,
}

impl Metadata {
    /// Creates a new [`Metadata`] with the specified properties
    pub(super) fn new(statx: &Statx) -> Self {
        let length = statx.size;

        let file_type = if s_isdir(statx.mode) {
            FileType::DIRECTORY
        } else if s_islnk(statx.mode) {
            FileType::SYMLINK
        } else {
            FileType::FILE
        };

        Metadata { file_type, length }
    }

    /// Gets the type of the file
    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    /// Is this file a file?
    pub fn is_file(&self) -> bool {
        self.file_type.is_file()
    }

    /// Is this file a directory?
    pub fn is_dir(&self) -> bool {
        self.file_type.is_dir()
    }

    /// Is this file a symbolic link?
    pub fn is_symlink(&self) -> bool {
        self.file_type.is_symlink()
    }

    /// Gets the length of the file in bytes
    pub fn len(&self) -> u64 {
        self.length
    }
}
