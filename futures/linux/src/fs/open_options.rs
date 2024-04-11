use std::ffi::c_int;

/// Options dictating access to a file
pub struct OpenOptions {
    options: c_int,
}
