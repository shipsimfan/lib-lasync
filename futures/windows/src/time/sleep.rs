use super::WaitableTimer;
use crate::EventRef;
use executor::EventManager;
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
pub fn sleep(duration: Duration) -> crate::Result<Sleep> {
    Sleep::new(duration)
}

impl Sleep {
    /// Creates a new [`Sleep`] which yields after `duration` has passed
    pub fn new(duration: Duration) -> crate::Result<Self> {
        let event_id = EventRef::register()?;

        let mut timer = WaitableTimer::new()?;
        timer.set(duration, None, *event_id)?;

        Ok(Sleep { timer, event_id })
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        EventManager::get_local_mut(|manager| {
            let event = manager.get_event_mut(*self.event_id).unwrap();

            if event.get_data() > 0 {
                return Poll::Ready(());
            }

            event.set_waker(Some(cx.waker().clone()));
            Poll::Pending
        })
    }
}
