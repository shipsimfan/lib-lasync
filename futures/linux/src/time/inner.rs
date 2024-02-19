use executor::Result;
use linux::{
    sys::timerfd::{timerfd_create, timerfd_settime},
    time::{itimerspec, timespec, CLOCK_MONOTONIC},
    try_linux,
    unistd::close,
};
use std::{ffi::c_int, ptr::null_mut, time::Duration};

/// A linux timerfd wrapper
pub(super) struct TimerFD {
    /// The file descriptor for the timer
    fd: c_int,

    /// Has this timer already been set?
    is_set: bool,
}

impl TimerFD {
    /// Creates a new [`WaitableTimer`]
    pub(super) fn new() -> Result<Self> {
        try_linux!(timerfd_create(CLOCK_MONOTONIC, 0)).map(|fd| TimerFD { fd, is_set: false })
    }

    /// Sets the timer to fire after `duration` and then every `interval` after.
    ///
    /// # Panic
    /// This function will panic if this timer is already set.
    pub(super) fn set(&mut self, duration: Duration, interval: Option<Duration>) -> Result<()> {
        assert!(!self.is_set);

        // Prepare the times
        let timerspec = itimerspec {
            interval: match interval {
                Some(interval) => timespec {
                    sec: interval.as_secs() as _,
                    nsec: interval.subsec_nanos() as _,
                },
                None => timespec::default(),
            },
            value: timespec {
                sec: duration.as_secs() as _,
                nsec: duration.subsec_nanos() as _,
            },
        };

        // Set the timer
        try_linux!(timerfd_settime(self.fd, 0, &timerspec, null_mut()))?;

        self.is_set = true;

        Ok(())
    }
}

impl Drop for TimerFD {
    fn drop(&mut self) {
        unsafe { close(self.fd) };
    }
}
