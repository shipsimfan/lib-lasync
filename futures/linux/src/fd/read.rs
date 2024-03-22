use crate::{AsFD, EventRef};
use executor::{platform::EventHandler, EventID, EventManager, Result};
use linux::Error;
use std::{
    ffi::c_int,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use uring::{io_uring_cqe, io_uring_prep_cancel64, io_uring_prep_read};

/// A future which yields aftering reading bytes from a [`Read`]
pub(crate) struct FDRead<'a, R: AsFD> {
    /// The source to read from
    source: &'a mut R,

    /// The event ID this is registered under
    event_id: Result<EventRef>,

    /// The buffer to read into
    buffer: &'a mut [u8],

    /// Has the SQE been submitted?
    sqe_submitted: bool,
}

/// The bit used to signal completion of the event in the value. 1 means finished. The bit must be
/// in a position greater than 32 to fit the result code from the read.
const SIGNAL_BIT: usize = 1 << 33;

/// The callback for when a read is completed
fn read_callback(cqe: &mut io_uring_cqe, value: &mut usize) {
    *value = (cqe.res as usize) | SIGNAL_BIT;
}

impl<'a, R: AsFD> FDRead<'a, R> {
    /// Creates a new [`FDRead`] future
    pub(crate) fn new(source: &'a mut R, buffer: &'a mut [u8]) -> Self {
        let event_id = EventRef::register(EventHandler::new(0, read_callback));

        FDRead {
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
    unsafe fn project(
        self: Pin<&mut Self>,
    ) -> (&mut R, Result<EventID>, Pin<&mut [u8]>, &mut bool) {
        let this = self.get_unchecked_mut();

        (
            this.source,
            this.event_id
                .as_ref()
                .map(|event_id| **event_id)
                .map_err(|error| *error),
            Pin::new(&mut this.buffer),
            &mut this.sqe_submitted,
        )
    }
}

impl<'a, R: AsFD> Future for FDRead<'a, R> {
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
                    io_uring_prep_read(
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
            let value = event.get_data().value();
            if value & SIGNAL_BIT == 0 {
                event.set_waker(Some(cx.waker().clone()));
                return Poll::Pending;
            }

            let bytes_read = (value & (u32::MAX as usize)) as c_int;
            if bytes_read < 0 {
                return Poll::Ready(Err(Error::new(-bytes_read)));
            }

            Poll::Ready(Ok(bytes_read as usize))
        })
    }
}

impl<'a, R: AsFD> Drop for FDRead<'a, R> {
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
