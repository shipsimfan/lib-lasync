use handler::SignalHandler;
use std::{ffi::c_int, sync::Arc};

mod handler;
mod signals;
mod value;

/// A signal which can be used by async I/O events
#[derive(Clone)]
pub(super) struct Signal(Arc<SignalHandler>);

impl Signal {
    /// Registers a signal handler on `signal_number`
    ///
    /// # Panic
    /// This function will panic if `signal_number` is not between 32 and 64 inclusive
    pub(super) fn register(signal_number: c_int) -> linux::Result<Self> {
        signals::update(signal_number, Self::register_inner)
    }

    /// Sets `signal` to a new [`Signal`] if it is [`None`]. Returns the signal in the slot
    fn register_inner(signal_number: c_int, signal: &mut Option<Signal>) -> linux::Result<Signal> {
        match signal {
            Some(signal) => Ok(signal.clone()),
            None => {
                let new_signal = Signal(Arc::new(SignalHandler::register(signal_number)?));
                *signal = Some(new_signal.clone());
                Ok(new_signal)
            }
        }
    }
}
