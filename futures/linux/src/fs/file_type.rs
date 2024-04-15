/// The type of a file
///
/// This uses integers internally instead of an enum to make this an opaque, unconstructable type.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FileType(u8);

/// The value representing a file
const FILE: u8 = 0;

/// The value representing a directory
const DIRECTORY: u8 = 1;

/// The value representing a symbolic link
const SYMLINK: u8 = 2;

impl FileType {
    /// The file is a file
    pub(super) const FILE: Self = FileType(FILE);

    /// The file is a directory
    pub(super) const DIRECTORY: Self = FileType(DIRECTORY);

    /// The file is a symlink
    pub(super) const SYMLINK: Self = FileType(SYMLINK);

    /// Is the file a file?
    pub fn is_file(&self) -> bool {
        self.0 == FILE
    }

    /// Is the file a directory?
    pub fn is_dir(&self) -> bool {
        self.0 == DIRECTORY
    }

    /// Is the file a symbolic link?
    pub fn is_symlink(&self) -> bool {
        self.0 == SYMLINK
    }
}
