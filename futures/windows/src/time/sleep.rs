use super::WaitableTimer;
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
    timer: WaitableTimer,

    /// The event id this is registered under
    event_id: EventRef,
}

/// Sleep until `duration` has passed
pub fn sleep(duration: Duration) -> Result<Sleep> {
    Sleep::new(duration)
}

/// Polls `event_id` (assuming it is a timer event) returning [`Poll::Ready`] when it triggers
pub(super) fn sleep_poll(event_id: EventID, cx: &mut Context) -> Poll<()> {
    EventManager::get_local_mut(|manager| {
        let event = manager.get_event_mut(event_id).unwrap();

        if event.get_data() > 0 {
            return Poll::Ready(());
        }

        event.set_waker(Some(cx.waker().clone()));
        Poll::Pending
    })
}

impl Sleep {
    /// Creates a new [`Sleep`] which yields after `duration` has passed
    pub fn new(duration: Duration) -> Result<Self> {
        let event_id = EventRef::register()?;

        let mut timer = WaitableTimer::new()?;
        timer.set(duration, None, *event_id)?;

        Ok(Sleep { timer, event_id })
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        sleep_poll(*self.event_id, cx)
    }
}
