use executor::Result;
use std::ptr::null_mut;
use win32::{try_get_last_error, CloseHandle, CreateEvent, FALSE, HANDLE};

/// A win32 event
pub(crate) struct Win32Event(HANDLE);

impl Win32Event {
    /// Creates an auto-resetting [`Win32Event`]
    pub(crate) fn new() -> Result<Self> {
        let event = try_get_last_error!(CreateEvent(null_mut(), FALSE, FALSE, null_mut()))?;

        Ok(Win32Event(event))
    }

    /// Gets the contained [`HANDLE`] for this event
    pub(crate) fn inner(&mut self) -> HANDLE {
        self.0
    }
}

impl Drop for Win32Event {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}
