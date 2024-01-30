use super::Signal;
use std::{ffi::c_int, sync::Mutex};

static SIGNALS: Mutex<[Option<Signal>; 33]> = Mutex::new([
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None,
]);

/// Updates the [`Signal`] associated with `signal_number`
///
/// # Panic
/// This function will panic if `signal_number` is not between 32 and 64 inclusive
pub(super) fn update<T, F: FnOnce(c_int, &mut Option<Signal>) -> T>(
    signal_number: c_int,
    f: F,
) -> T {
    assert!(signal_number >= 32);
    assert!(signal_number <= 64);

    let index = signal_number as usize - 32;
    let mut signals = SIGNALS.lock().unwrap();
    f(signal_number, &mut signals[index])
}
