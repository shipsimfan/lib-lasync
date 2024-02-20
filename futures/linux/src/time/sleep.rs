use super::TimerFD;
use crate::EventRef;
use executor::{EventID, EventManager, Result};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// A future which yields after a certain duration
pub struct Sleep {
    /// The timer that will fire when the sleep ends
    #[allow(unused)]
    timer: TimerFD,

    /// The event id this is registered under
    event_id: EventRef,

    /// Has the [`io_uring_sqe`] been submitted yet?
    sqe_submitted: bool,
}

/// Sleep until `duration` has passed
pub fn sleep(duration: Duration) -> Result<Sleep> {
    Sleep::new(duration)
}

impl Sleep {
    /// Creates a new [`Sleep`] which yields after `duration` has passed
    pub fn new(duration: Duration) -> Result<Self> {
        let event_id = EventRef::register(TimerFD::HANDLER)?;

        let mut timer = TimerFD::new()?;
        timer.set(duration, None)?;

        Ok(Sleep {
            timer,
            event_id,
            sqe_submitted: false,
        })
    }

    /// Submits the sqe for this event
    fn submit_sqe(self: Pin<&mut Self>) -> Result<()> {
        let (timer, sqe_submitted, event_id) = self.project();
        assert!(!*sqe_submitted);

        timer.submit_sqe(event_id)?;

        *sqe_submitted = true;
        Ok(())
    }

    /// Projects this to `(self.timer, self.sqe_submitted, self.event_id)`
    fn project(self: Pin<&mut Self>) -> (Pin<&mut TimerFD>, &mut bool, EventID) {
        let this = unsafe { self.get_unchecked_mut() };
        (
            Pin::new(&mut this.timer),
            &mut this.sqe_submitted,
            *this.event_id,
        )
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.sqe_submitted {
            self.as_mut().submit_sqe().unwrap();
        }

        EventManager::get_local_mut(|manager| {
            let event = manager.get_event_mut(*self.event_id).unwrap();

            if event.get_data().value() > 0 {
                return Poll::Ready(());
            }

            event.set_waker(Some(cx.waker().clone()));
            Poll::Pending
        })
    }
}
