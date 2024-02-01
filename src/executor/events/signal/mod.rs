use linux::{
    signal::{sigaction, sigaction_handler, sigaction_t, SA_SIGINFO, SIGUSR1},
    try_linux,
};
use std::{ffi::c_int, ptr::null_mut, sync::Once};

mod handler;
mod value;

pub use value::SignalValue;

/// Verifies the signal handler is only installed once
static SIGNAL_HANDLER: Once = Once::new();

pub(super) const SIGNAL_NUMBER: c_int = SIGUSR1;

/// Registers the signal handler using [`sigaction`] into [`SIGNAL_NUMBER`]
pub(super) fn register() -> linux::Result<()> {
    let mut result = 0;
    SIGNAL_HANDLER.call_once(|| result = do_register());
    try_linux!(result).map(|_| ())
}

/// Actually registers the signal handler
fn do_register() -> i32 {
    let act = sigaction_t {
        handler: sigaction_handler {
            sigaction: Some(handler::signal_handler),
        },
        flags: SA_SIGINFO,
        ..Default::default()
    };

    unsafe { sigaction(SIGNAL_NUMBER, &act, null_mut()) }
}
