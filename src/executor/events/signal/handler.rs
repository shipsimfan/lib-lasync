use crate::executor::EventTrigger;
use linux::signal::siginfo_t;
use std::{
    ffi::{c_int, c_void},
    ptr::null_mut,
};

/// Handles signals for the event manager
pub(super) extern "C" fn signal_handler(_: c_int, siginfo: *mut siginfo_t, _: *mut c_void) {
    // Convert `siginfo` into an `EventTrigger`
    if siginfo == null_mut() {
        return;
    }
    let siginfo = unsafe { &*siginfo };

    let sigval = unsafe { siginfo.fields.rt.sigval.ptr };
    if sigval == null_mut() {
        return;
    }
    let trigger = unsafe { &*(sigval as *const EventTrigger) };

    // Trigger the event
    trigger.trigger();
}
