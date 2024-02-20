use executor::{platform::EventHandler, EventID, EventManager, Result};
use linux::{
    sys::timerfd::{timerfd_create, timerfd_settime},
    time::{itimerspec, timespec, CLOCK_MONOTONIC},
    try_linux,
    unistd::close,
};
use std::{ffi::c_int, pin::Pin, ptr::null_mut, time::Duration};
use uring::{io_uring_cqe, io_uring_prep_read};

/// A linux timerfd wrapper
pub(super) struct TimerFD {
    /// The file descriptor for the timer
    fd: c_int,

    /// Has this timer already been set?
    is_set: bool,

    /// The buffer for the output to be placed in
    read_buffer: u64,
}

/// The callback for when timers trigger
fn timer_callback(cqe: &mut io_uring_cqe, value: &mut usize) {
    *value += 1;
}

impl TimerFD {
    /// The [`EventHandler`] for all timer events
    pub(super) const HANDLER: EventHandler = EventHandler::new(0, timer_callback);

    /// Creates a new [`WaitableTimer`]
    pub(super) fn new() -> Result<Self> {
        try_linux!(timerfd_create(CLOCK_MONOTONIC, 0)).map(|fd| TimerFD {
            fd,
            is_set: false,
            read_buffer: 0,
        })
    }

    /// Submits a read sqe for this event
    pub(super) fn submit_sqe(self: Pin<&mut Self>, event_id: EventID) -> Result<()> {
        let (read_buffer, fd) = self.project();

        EventManager::get_local_mut(|manager| {
            let sqe = manager.get_sqe()?;

            unsafe { io_uring_prep_read(sqe, fd, read_buffer.get_mut() as *mut _ as _, 8, 0) };

            manager.submit_sqe(sqe, event_id)?;

            Ok(())
        })
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

    ///Projects this to `(self.read_buffer, self.fd)`
    fn project(self: Pin<&mut Self>) -> (Pin<&mut u64>, c_int) {
        let this = unsafe { self.get_unchecked_mut() };

        (Pin::new(&mut this.read_buffer), this.fd)
    }
}

impl Drop for TimerFD {
    fn drop(&mut self) {
        unsafe { close(self.fd) };
    }
}
