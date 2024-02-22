use crate::EventRef;
use executor::{platform::EventHandler, EventID, EventManager, Result};
use linux::time::__kernel_timespec;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use uring::{io_uring_cqe, io_uring_prep_timeout};

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
pub struct Tick<'a>(&'a mut Interval);

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
        let event_id = EventRef::register(EventHandler::new(0, interval_callback))?;

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
        Tick(self)
    }

    /// Projects this object into `(self.timespec, self.sqe_submitted, self.event_id)`
    pub(self) fn project(
        self: Pin<&mut Self>,
    ) -> (Pin<&mut __kernel_timespec>, &mut bool, EventID) {
        let this = unsafe { self.get_unchecked_mut() };
        (
            Pin::new(&mut this.timespec),
            &mut this.sqe_submitted,
            *this.event_id,
        )
    }
}

impl<'a> Tick<'a> {
    fn project(self: Pin<&mut Self>) -> Pin<&mut Interval> {
        Pin::new(&mut self.get_mut().0)
    }
}

impl<'a> Future for Tick<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (timespec, sqe_submitted, event_id) = self.project().project();

        EventManager::get_local_mut(|manager| {
            // Submit the SQE if one hasn't been submitted yet
            if !*sqe_submitted {
                let sqe = manager.get_sqe(event_id).unwrap();

                unsafe {
                    io_uring_prep_timeout(
                        sqe.as_ptr(),
                        timespec.get_mut(),
                        0,
                        todo!("IORING_TIMEOUT_MULTISHOT"),
                    )
                };

                sqe.submit().unwrap();
                *sqe_submitted = true;
            }

            // Check if the event is ready
            let event = manager.get_event_mut(event_id).unwrap();
            let value = event.get_data().value();
            if value > 0 {
                event.data_mut().set_value(value - 1);
                return Poll::Ready(());
            }

            event.set_waker(Some(cx.waker().clone()));
            Poll::Pending
        })
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        todo!("io_uring_prep_timeout_remove");
    }
}
