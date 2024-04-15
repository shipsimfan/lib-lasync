use crate::{
    event_ref::EventRef,
    fd::AsFD,
    fs::{File, Metadata},
};
use executor::{
    platform::{
        linux::{
            fcntl::{AT_EMPTY_PATH, AT_NO_AUTOMOUNT},
            sys::stat::{Statx, STATX_BASIC_STATS},
        },
        uring::{io_uring_cqe, io_uring_prep_cancel64, io_uring_prep_statx},
        EventHandler,
    },
    Error, EventID, EventManager, Result,
};
use std::{
    ffi::c_int,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

// rustdoc imports
#[allow(unused_imports)]
use executor::platform::linux::sys::stat::statx;

/// A [`Future`] which yields the [`Metadata`] for an open [`File`]
pub struct FileStat<'a> {
    /// The [`File`] to get [`Metadata`] for
    file: &'a File,

    /// The buffer for the output of the [`statx`] call
    buffer: Statx,

    /// The event ID this is registered under
    event_id: Result<EventRef>,

    /// Has the SQE been submitted?
    sqe_submitted: bool,
}

/// The bit used to signal completion of the event in the value. 1 means finished. The bit must be
/// in a position greater than 32 to fit the result code from [`statx`].
const SIGNAL_BIT: usize = 1 << 33;

/// The callback for when a stat is completed
fn stat_callback(cqe: &mut io_uring_cqe, value: &mut usize) {
    *value = (cqe.res as usize) | SIGNAL_BIT;
}

impl<'a> FileStat<'a> {
    /// Creates a new [`FileStat`] [`Future`]
    pub(super) fn new(file: &'a File) -> Self {
        let event_id = EventRef::register(EventHandler::integer(stat_callback));

        FileStat {
            file,
            buffer: Statx::default(),
            event_id,
            sqe_submitted: false,
        }
    }

    /// Projects pinned self into `(self.file, self.event_id, self.buffer, self.sqe_submitted)`
    ///
    /// # SAFTEY
    /// This is the only way to access the contained `buffer`, do not access it directly.
    unsafe fn project(
        self: Pin<&mut Self>,
    ) -> (&File, Result<EventID>, Pin<&mut Statx>, &mut bool) {
        let this = self.get_unchecked_mut();

        (
            this.file,
            this.event_id
                .as_ref()
                .map(|event_id| **event_id)
                .map_err(|error| *error),
            Pin::new(&mut this.buffer),
            &mut this.sqe_submitted,
        )
    }
}

impl<'a> Future for FileStat<'a> {
    type Output = Result<Metadata>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (file, event_id, buffer, sqe_submitted) = unsafe { self.project() };

        let event_id = match event_id {
            Ok(event_id) => event_id,
            Err(error) => return Poll::Ready(Err(error)),
        };

        EventManager::get_local_mut(|manager| {
            let buffer = buffer.get_mut();

            // Submit the SQE if one hasn't been submitted yet
            if !*sqe_submitted {
                let sqe = manager.get_sqe(event_id).unwrap();

                unsafe {
                    io_uring_prep_statx(
                        sqe.as_ptr(),
                        file.fd(),
                        b"\0".as_ptr().cast(),
                        AT_EMPTY_PATH | AT_NO_AUTOMOUNT,
                        STATX_BASIC_STATS as _,
                        buffer,
                    )
                }

                sqe.submit().unwrap();
                *sqe_submitted = true;
            }

            let event = manager.get_event_mut(event_id).unwrap();
            let value = event.data().as_integer();
            if value & SIGNAL_BIT == 0 {
                event.set_waker(Some(cx.waker().clone()));
                return Poll::Pending;
            }

            *sqe_submitted = false;

            let result = (value & (u32::MAX as usize)) as c_int;
            if result < 0 {
                return Poll::Ready(Err(Error::new(-result)));
            }

            Poll::Ready(Ok(Metadata::new(buffer)))
        })
    }
}

impl<'a> Drop for FileStat<'a> {
    fn drop(&mut self) {
        if let Ok(event_id) = &self.event_id {
            if self.sqe_submitted {
                EventManager::get_local_mut(|manager| {
                    let sqe = manager.get_sqe(**event_id).unwrap();

                    unsafe { io_uring_prep_cancel64(sqe.as_ptr(), (**event_id).into_u64(), 0) };

                    sqe.submit().unwrap();
                })
            }
        }
    }
}
