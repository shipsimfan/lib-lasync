use crate::EventRef;
use executor::{platform::EventHandler, EventManager, Result};
use linux::time::__kernel_timespec;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use uring::{
    io_uring_cqe, io_uring_prep_timeout, io_uring_prep_timeout_remove, IORING_TIMEOUT_MULTISHOT,
};

/// A future which yields after a fixed period
pub struct Interval {
    /// The timespec for the SQE
    timespec: __kernel_timespec,

    /// The event id this is registered under
    event_id: EventRef,

    /// Has the SQE been submitted yet?
    sqe_submitted: bool,
}

/// A future which yields after one tick from an [`Interval`]
pub struct Tick<'a> {
    timespec: __kernel_timespec,
    interval: &'a mut Interval,
}

/// Creates an [`Interval`] future which yields immediately then yields every `period`
pub fn interval(period: Duration) -> Result<Interval> {
    Interval::new(period)
}

/// The callback called when the interval timer fires
fn interval_callback(_: &mut io_uring_cqe, value: &mut usize) {
    *value += 1;
}

impl Interval {
    /// Creates a new [`Interval`] that first yields after `delay` and then yields every `period`
    pub fn new(period: Duration) -> Result<Self> {
        let event_id = EventRef::register(EventHandler::integer(interval_callback))?;

        let timespec = __kernel_timespec {
            sec: period.as_secs() as _,
            nsec: period.subsec_nanos() as _,
        };

        Ok(Interval {
            timespec,
            event_id,
            sqe_submitted: false,
        })
    }

    /// Returns a future which will yield after the next timer tick
    pub fn tick(&mut self) -> Tick {
        Tick {
            timespec: self.timespec,
            interval: self,
        }
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        if self.sqe_submitted {
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

impl !Send for Interval {}
impl !Sync for Interval {}

impl<'a> Tick<'a> {
    /// Projects this into `(self.timespec, self.interval)`
    ///
    /// # SAFTEY
    /// This is the only way to access the contained [`__kernel_timespec`], do not acces it
    /// directly.
    unsafe fn project(self: Pin<&mut Self>) -> (Pin<&mut __kernel_timespec>, &mut Interval) {
        let this = self.get_unchecked_mut();
        (Pin::new(&mut this.timespec), this.interval)
    }
}

impl<'a> Future for Tick<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (timespec, interval) = unsafe { self.project() };

        EventManager::get_local_mut(|manager| {
            // Submit the SQE if one hasn't been submitted yet
            if !interval.sqe_submitted {
                let sqe = manager.get_sqe(*interval.event_id).unwrap();

                unsafe {
                    io_uring_prep_timeout(
                        sqe.as_ptr(),
                        timespec.get_mut(),
                        0,
                        IORING_TIMEOUT_MULTISHOT,
                    )
                };

                sqe.submit().unwrap();
                interval.sqe_submitted = true;
            }

            // Check if the event is ready
            let event = manager.get_event_mut(*interval.event_id).unwrap();
            let value = event.get_data().as_integer();
            if value > 0 {
                event.data_mut().set_integer(value - 1);
                return Poll::Ready(());
            }

            event.set_waker(Some(cx.waker().clone()));
            Poll::Pending
        })
    }
}

impl<'a> !Send for Tick<'a> {}
impl<'a> !Sync for Tick<'a> {}
