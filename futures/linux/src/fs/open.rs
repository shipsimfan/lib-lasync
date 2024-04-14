use crate::{fs::File, EventRef};
use executor::{
    platform::{
        linux::errno::EINVAL,
        uring::{io_uring_cqe, io_uring_prep_openat},
        EventHandler,
    },
    Error, EventManager, Result,
};
use std::{
    ffi::{c_int, CString},
    future::Future,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

/// A [`Future`] which yields when a file open is complete
pub struct Open {
    /// The path to open
    path: Result<CString>,

    /// The options to open with
    options: Result<c_int>,

    /// The event ID this is registered under
    event_id: Result<EventRef>,

    /// Has the SQE been submitted?
    sqe_submitted: bool,
}

/// The bit used to signal completion of the event in the value. 1 means finished. The bit must be
/// in a position greater than 32 to fit the result code from the read.
const SIGNAL_BIT: usize = 1 << 33;

/// The callback for when an open is completed
fn open_callback(cqe: &mut io_uring_cqe, value: &mut usize) {
    *value = (cqe.res as usize) | SIGNAL_BIT;
}

impl Open {
    /// Creates a new [`Open`] [`Future`] to open the file at `path` with `options`
    pub(super) fn new(path: &Path, options: Result<c_int>) -> Self {
        let path =
            CString::new(path.as_os_str().as_encoded_bytes()).map_err(|_| Error::new(EINVAL));

        let event_id = EventRef::register(EventHandler::integer(open_callback));

        Open {
            path,
            options,
            event_id,
            sqe_submitted: false,
        }
    }
}

impl Future for Open {
    type Output = Result<File>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let event_id = match &self.event_id {
            Ok(event_id) => **event_id,
            Err(error) => return Poll::Ready(Err(*error)),
        };

        EventManager::get_local_mut(|manager| {
            // Submit the SQE if one hasn't been submitted yet
            if !self.sqe_submitted {
                let dfd = match unsafe { manager.working_directory() } {
                    Ok(dfd) => dfd,
                    Err(error) => return Poll::Ready(Err(error)),
                };

                let path = match self.path.as_ref() {
                    Ok(path) => path.as_ptr(),
                    Err(error) => return Poll::Ready(Err(*error)),
                };

                let options = match self.options {
                    Ok(options) => options,
                    Err(error) => return Poll::Ready(Err(error)),
                };

                let sqe = manager.get_sqe(event_id).unwrap();

                unsafe { io_uring_prep_openat(sqe.as_ptr(), dfd, path, options, 0o777) };

                sqe.submit().unwrap();
                self.sqe_submitted = true;
            }

            let event = manager.get_event_mut(event_id).unwrap();
            let value = event.data().as_integer();
            if value & SIGNAL_BIT == 0 {
                event.set_waker(Some(cx.waker().clone()));
                return Poll::Pending;
            }

            let result = (value & (u32::MAX as usize)) as c_int;
            if result < 0 {
                return Poll::Ready(Err(Error::new(-result)));
            }

            Poll::Ready(Ok(File::new(result)))
        })
    }
}
