use linux::{
    sys::timerfd::{timerfd_create, timerfd_settime, TFD_NONBLOCK},
    time::{itimerspec, timespec, CLOCK_MONOTONIC},
    try_linux,
    unistd::close,
};
use std::{
    ffi::c_int,
    future::Future,
    pin::Pin,
    ptr::null_mut,
    task::{Context, Poll},
    time::Duration,
};

/// A future that signals after a certain duration
pub struct Timer {
    /// The file descriptor for the timer
    file_descriptor: c_int,

    /// Has this timer been register with the event manager?
    registered: bool,
}

impl Timer {
    /// Creates and starts a new [`Timer`] for `duration`
    pub fn new(duration: Duration) -> linux::Result<Self> {
        let file_descriptor = try_linux!(timerfd_create(CLOCK_MONOTONIC, TFD_NONBLOCK))?;

        try_linux!(timerfd_settime(
            file_descriptor,
            0,
            &itimerspec {
                interval: timespec { sec: 0, nsec: 0 },
                value: timespec {
                    sec: duration.as_secs() as _,
                    nsec: duration.subsec_nanos() as _
                }
            },
            null_mut()
        ))?;

        Ok(Timer {
            file_descriptor,
            registered: false,
        })
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if the timer is expired using `timerfd_gettime`

        // If it is expired,
        //     Deregister the event
        //     Return `Poll::Ready`

        // Otherwise, (re)register the timer with the executor

        todo!()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if self.registered {
            // Deregister the event
        }

        // Close the descriptor
        unsafe { close(self.file_descriptor) };
    }
}
