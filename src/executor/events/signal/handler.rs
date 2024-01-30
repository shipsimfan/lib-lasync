use std::ffi::c_int;

/// A registered signal handler
pub(super) struct SignalHandler(c_int);

/// Handles signals for the event manager
extern "C" fn signal_handler() {
    todo!()
}

impl SignalHandler {
    /// Registers a signal handler using [`sigaction`]
    pub(super) fn register(signal_number: c_int) -> linux::Result<Self> {
        todo!("Register using `sigaction`")
    }
}

impl Drop for SignalHandler {
    fn drop(&mut self) {
        todo!("Deregister the signal")
    }
}
