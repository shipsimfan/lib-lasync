use crate::Result;
use executor::{EventID, EventManager};
use std::{
    ptr::{null, null_mut},
    time::Duration,
};
use win32::{
    try_get_last_error, CancelWaitableTimer, CloseHandle, CreateWaitableTimer, SetWaitableTimer,
    DWORD, FALSE, HANDLE, LARGE_INTEGER, LONG, LPVOID, TRUE,
};

/// A Windows waitable timer
pub(super) struct WaitableTimer {
    /// The handle to the timer
    handle: HANDLE,

    /// Has this timer already been set?
    is_set: bool,
}

/// The function which is called when a timer fires
///
/// This function increments the `data` value associated with `event_id`
extern "system" fn timer_apc(event_id: LPVOID, _: DWORD, _: DWORD) {
    let event_id = unsafe { EventID::from_u64(event_id as u64) };

    EventManager::get_local_mut(|manager| {
        let event = match manager.get_event_mut(event_id) {
            Some(event) => event,
            None => return,
        };

        *event.data_mut() += 1;
        event.wake();
    });
}

impl WaitableTimer {
    /// Creates a new [`WaitableTimer`]
    pub(super) fn new() -> Result<Self> {
        try_get_last_error!(CreateWaitableTimer(null_mut(), FALSE, null())).map(|handle| {
            WaitableTimer {
                handle,
                is_set: false,
            }
        })
    }

    /// Sets the timer to fire after `duration` and then every `interval` after.
    ///
    /// Upon firing, the timer will increment the value associated with the `event_id` event.
    ///
    /// # Panic
    /// This function will panic if this timer is already set.
    pub(super) fn set(
        &mut self,
        duration: Duration,
        interval: Option<Duration>,
        event_id: EventID,
    ) -> Result<()> {
        assert!(!self.is_set);

        // Prepare the times
        let due_time = LARGE_INTEGER {
            quad_part: -((duration.as_nanos() / 100) as i64),
        };

        let period = interval
            .map(|interval| interval.as_millis() as LONG)
            .unwrap_or(0);

        // Set the timer
        try_get_last_error!(SetWaitableTimer(
            self.handle,
            &due_time,
            period,
            timer_apc,
            event_id.into_u64() as _,
            TRUE
        ))?;

        self.is_set = true;

        Ok(())
    }

    /// Cancels the outstanding timer event
    ///
    /// # Panic
    /// This function will panic if this timer is not already set.
    pub(super) fn cancel(&mut self) -> Result<()> {
        assert!(self.is_set);

        try_get_last_error!(CancelWaitableTimer(self.handle))?;

        self.is_set = false;

        Ok(())
    }
}

impl Drop for WaitableTimer {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}
