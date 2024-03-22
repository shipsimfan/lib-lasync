use std::ffi::c_int;

/// An object which has an underlying file descriptor
pub(crate) trait AsFD {
    /// Gets the underlying file descriptor
    unsafe fn fd(&self) -> c_int;
}
