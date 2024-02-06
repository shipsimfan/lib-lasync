use linux::{
    sys::epoll::{epoll_create, epoll_ctl, epoll_event, epoll_wait, EPOLL_CTL_ADD, EPOLL_CTL_DEL},
    try_linux,
    unistd::close,
};
use std::{ffi::c_int, ptr::null};

/// An epoll object for polling multiple file descriptors
pub(super) struct EPoll(c_int);

impl EPoll {
    /// Creates a new [`EPoll`]
    pub(super) fn new() -> linux::Result<Self> {
        let descriptor = try_linux!(epoll_create(1))?;

        Ok(EPoll(descriptor))
    }

    /// Registers `fd` to be polled for `events`
    pub(super) fn register_fd(&mut self, fd: c_int, events: u32, data: u64) -> linux::Result<()> {
        let epoll_event = epoll_event {
            events,
            data: linux::sys::epoll::epoll_data_t { u64: data },
        };

        try_linux!(epoll_ctl(self.0, EPOLL_CTL_ADD, fd, &epoll_event)).map(|_| ())
    }

    /// Polls for ready events, blocking until one is ready if `block` is true
    pub(super) fn poll(&mut self, block: bool) -> linux::Result<Option<u64>> {
        let mut event = epoll_event::default();

        let count = try_linux!(epoll_wait(
            self.0,
            &mut event,
            1,
            if block { -1 } else { 0 }
        ))?;

        Ok(if count > 0 {
            Some(unsafe { event.data.u64 })
        } else {
            None
        })
    }

    /// Removes `fd` from the list of descriptors to be polled
    pub(super) fn unregister_fd(&mut self, fd: c_int) -> linux::Result<()> {
        try_linux!(epoll_ctl(self.0, EPOLL_CTL_DEL, fd, null())).map(|_| ())
    }
}

impl Drop for EPoll {
    fn drop(&mut self) {
        unsafe { close(self.0) };
    }
}
