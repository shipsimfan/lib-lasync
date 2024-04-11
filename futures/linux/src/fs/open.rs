use crate::fs::File;
use executor::{Error, Result};
use linux::errno::EINVAL;
use std::{
    ffi::{c_int, CString},
    future::Future,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

/// A [`Future`] which yields when a file open is complete
pub struct Open {
    /// The path to open
    path: Result<CString>,
}

impl Open {
    /// Creates a new [`Open`] [`Future`] to open the file at `path` with `options`
    pub(super) fn new(path: &Path, options: c_int) -> Self {
        let path =
            CString::new(path.as_os_str().as_encoded_bytes()).map_err(|_| Error::new(EINVAL));

        todo!("Open::new()")
    }
}

impl Future for Open {
    type Output = Result<File>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("Open::poll()")
    }
}
