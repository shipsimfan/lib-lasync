use crate::executor::EventManager;
use linux::{
    sys::{
        epoll::EPOLLIN,
        timerfd::{timerfd_create, timerfd_settime, TFD_NONBLOCK},
    },
    time::{itimerspec, timespec, CLOCK_MONOTONIC},
    try_linux,
    unistd::{close, read},
};
use std::{
    cell::UnsafeCell,
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
    registered: UnsafeCell<bool>,
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
            registered: UnsafeCell::new(false),
        })
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let registered = unsafe { &mut *self.registered.get() };
        *registered = false;

        // Check if the timer is expired using `read`
        let mut buffer = [0; 8];
        let result = try_linux!(read(
            self.file_descriptor,
            buffer.as_mut_ptr().cast(),
            buffer.len() as _
        ));

        // According to the man pages, the only error the above `read` should return is `EAGAIN`
        let expired = if result.is_err() {
            false
        // According to the man pages, this shouldn't happen, but it's here just in case it does
        } else if buffer == [0; 8] {
            false
        } else {
            true
        };

        if expired {
            return Poll::Ready(());
        }

        EventManager::register(self.file_descriptor, EPOLLIN as _, cx.waker().clone()).unwrap();
        *registered = true;

        Poll::Pending
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if unsafe { *self.registered.get() } {
            EventManager::unregister(self.file_descriptor).ok();
        }

        // Close the descriptor
        unsafe { close(self.file_descriptor) };
    }
}
