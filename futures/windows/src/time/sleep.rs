use super::WaitableTimer;
use crate::EventRef;
use executor::{EventID, EventManager};
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
pub fn sleep(duration: Duration) -> win32::Result<Sleep> {
    Sleep::new(duration)
}

/// Creates and starts a timer to a given duration
fn create_and_set_timer(duration: Duration, event_id: EventID) -> crate::Result<WaitableTimer> {
    let mut timer = WaitableTimer::new()?;
    timer.set(duration, None, event_id)?;
    Ok(timer)
}

/// Attempts to register an event with the local event manager
fn register() -> crate::Result<EventID> {
    EventManager::get_local_mut(|manager| manager.register(0))
        .ok_or(win32::Error::new(win32::ERROR_TOO_MANY_CMDS))
}

impl Sleep {
    /// Creates a new [`Sleep`] which yields after `duration` has passed
    pub fn new(duration: Duration) -> crate::Result<Self> {
        let event_id = EventRef::new(register()?);

        let timer = create_and_set_timer(duration, *event_id)?;

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
