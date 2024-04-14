use crate::{AsFD, EventRef};
use executor::{
    platform::{
        uring::{io_uring_cqe, io_uring_prep_cancel64, io_uring_prep_write},
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

/// A future which yields aftering writing bytes to a [`Write`]
pub(crate) struct FDWrite<'a, W: AsFD> {
    /// The source to read from
    source: &'a mut W,

    /// The event ID this is registered under
    event_id: Result<EventRef>,

    /// The buffer to write from
    buffer: &'a [u8],

    /// Has the SQE been submitted?
    sqe_submitted: bool,
}

/// The bit used to signal completion of the event in the value. 1 means finished. The bit must be
/// in a position greater than 32 to fit the result code from the read.
const SIGNAL_BIT: usize = 1 << 33;

/// The callback for when a write is completed
fn write_callback(cqe: &mut io_uring_cqe, value: &mut usize) {
    *value = (cqe.res as usize) | SIGNAL_BIT;
}

impl<'a, W: AsFD> FDWrite<'a, W> {
    /// Creates a new [`FDWrite`] future
    pub(crate) fn new(source: &'a mut W, buffer: &'a [u8]) -> Self {
        let event_id = EventRef::register(EventHandler::integer(write_callback));

        FDWrite {
            source,
            event_id,
            buffer,
            sqe_submitted: false,
        }
    }

    /// Projects pinned self into `(self.source, self.event_id, self.buffer, self.sqe_submitted)`
    ///
    /// # SAFTEY
    /// This is the only way to access the contained `buffer`, do not access it directly.
    unsafe fn project(self: Pin<&mut Self>) -> (&mut W, Result<EventID>, Pin<&[u8]>, &mut bool) {
        let this = self.get_unchecked_mut();

        (
            this.source,
            this.event_id
                .as_ref()
                .map(|event_id| **event_id)
                .map_err(|error| *error),
            Pin::new(&this.buffer),
            &mut this.sqe_submitted,
        )
    }
}

impl<'a, W: AsFD> Future for FDWrite<'a, W> {
    type Output = Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (source, event_id, buffer, sqe_submitted) = unsafe { self.project() };

        let event_id = match event_id {
            Ok(event_id) => event_id,
            Err(error) => return Poll::Ready(Err(error)),
        };

        EventManager::get_local_mut(|manager| {
            // Submit the SQE if one hasn't been submitted yet
            if !*sqe_submitted {
                let sqe = manager.get_sqe(event_id).unwrap();

                unsafe {
                    io_uring_prep_write(
                        sqe.as_ptr(),
                        source.fd(),
                        buffer.as_ptr() as _,
                        buffer.len() as _,
                        u64::MAX,
                    )
                };

                sqe.submit().unwrap();
                *sqe_submitted = true;
            }

            let event = manager.get_event_mut(event_id).unwrap();
            let value = event.data().as_integer();
            if value & SIGNAL_BIT == 0 {
                event.set_waker(Some(cx.waker().clone()));
                return Poll::Pending;
            }

            let bytes_written = (value & (u32::MAX as usize)) as c_int;
            if bytes_written < 0 {
                return Poll::Ready(Err(Error::new(-bytes_written)));
            }

            Poll::Ready(Ok(bytes_written as usize))
        })
    }
}

impl<'a, W: AsFD> Drop for FDWrite<'a, W> {
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

impl<'a, W: AsFD> !Send for FDWrite<'a, W> {}
impl<'a, W: AsFD> !Sync for FDWrite<'a, W> {}
