use linux::signal::siginfo_t;
use std::ffi::{c_int, c_void};

/// Handles signals for the event manager
pub(super) extern "C" fn signal_handler(_: c_int, siginfo: *mut siginfo_t, _: *mut c_void) {
    todo!("Convert `sigvalue` to `SignalValue`");

    todo!("Trigger the `SignalValue`");
}
