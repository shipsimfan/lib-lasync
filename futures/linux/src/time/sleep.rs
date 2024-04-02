use crate::EventRef;
use executor::{platform::EventHandler, EventID, EventManager, Result};
use linux::time::__kernel_timespec;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use uring::{io_uring_cqe, io_uring_prep_timeout, io_uring_prep_timeout_remove};

/// A future which yields after a certain duration
pub struct Sleep {
    /// The timespec for the SQE
    timespec: __kernel_timespec,

    /// The event id this is registered under
    event_id: EventRef,

    /// Has the SQE been submitted yet?
    sqe_submitted: bool,

    /// Has the sleep completed?
    completed: bool,
}

/// Sleep until `duration` has passed
pub fn sleep(duration: Duration) -> Result<Sleep> {
    Sleep::new(duration)
}

/// The callback called when the sleep timer fires
fn sleep_callback(_: &mut io_uring_cqe, value: &mut usize) {
    *value += 1;
}

impl Sleep {
    /// Creates a new [`Sleep`] which yields after `duration` has passed
    pub fn new(duration: Duration) -> Result<Self> {
        let event_id = EventRef::register(EventHandler::integer(sleep_callback))?;

        let timespec = __kernel_timespec {
            sec: duration.as_secs() as _,
            nsec: duration.subsec_nanos() as _,
        };

        Ok(Sleep {
            timespec,
            event_id,
            sqe_submitted: false,
            completed: false,
        })
    }

    /// Projects this object into
    /// `(self.timespec, self.sqe_submitted, self.completed, self.event_id)`
    ///
    /// # SAFTEY
    /// This is the only way to access the contained [`__kernel_timespec`], do not acces it
    /// directly.
    unsafe fn project(
        self: Pin<&mut Self>,
    ) -> (Pin<&mut __kernel_timespec>, &mut bool, &mut bool, EventID) {
        let this = self.get_unchecked_mut();
        (
            Pin::new(&mut this.timespec),
            &mut this.sqe_submitted,
            &mut this.completed,
            *this.event_id,
        )
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (timespec, sqe_submitted, completed, event_id) = unsafe { self.project() };

        EventManager::get_local_mut(|manager| {
            // Submit the SQE if one hasn't been submitted yet
            if !*sqe_submitted {
                let sqe = manager.get_sqe(event_id).unwrap();

                unsafe { io_uring_prep_timeout(sqe.as_ptr(), timespec.get_mut(), 0, 0) };

                sqe.submit().unwrap();
                *sqe_submitted = true;
            }

            // Check if the event is ready
            let event = manager.get_event_mut(event_id).unwrap();
            if event.get_data().as_integer() > 0 {
                *completed = true;
                return Poll::Ready(());
            }

            event.set_waker(Some(cx.waker().clone()));
            Poll::Pending
        })
    }
}

impl Drop for Sleep {
    fn drop(&mut self) {
        if self.sqe_submitted && !self.completed {
            EventManager::get_local_mut(|manager| {
                let sqe = manager.get_sqe(*self.event_id).unwrap();

                unsafe {
                    io_uring_prep_timeout_remove(sqe.as_ptr(), (*self.event_id).into_u64(), 0)
                };

                sqe.submit().unwrap();
            })
        }
    }
}

impl !Send for Sleep {}
impl !Sync for Sleep {}
