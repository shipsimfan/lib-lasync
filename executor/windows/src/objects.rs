use executor_common::EventID;
use std::ptr::null_mut;
use win32::{
    Error, Result, WaitForMultipleObjectsEx, DWORD, ERROR_TOO_MANY_CMDS, FALSE, HANDLE,
    MAXIMUM_WAIT_OBJECTS, TRUE, WAIT_FAILED, WAIT_IO_COMPLETION, WAIT_TIMEOUT,
};

/// The result of waiting for an object
pub(crate) enum WaitResult {
    /// An object signalled
    Object(EventID),

    /// An I/O completion routine ran
    IOCompletion,

    /// The wait function timed-out
    Timeout,
}

/// Maintains a list of objects to wait on
pub(crate) struct Objects {
    /// The handles to wait on
    handles: [HANDLE; MAXIMUM_WAIT_OBJECTS as usize],

    /// The events to wake when a handle is signalled
    event_ids: [Option<EventID>; MAXIMUM_WAIT_OBJECTS as usize],

    /// The number of objects currently in the list
    count: usize,
}

impl Objects {
    /// Creates an empty list of [`Objects`]
    pub(crate) fn new() -> Self {
        Objects {
            handles: [null_mut(); MAXIMUM_WAIT_OBJECTS as usize],
            event_ids: [None; MAXIMUM_WAIT_OBJECTS as usize],
            count: 0,
        }
    }

    /// Gets the number of objects currently in the list
    pub(super) fn count(&self) -> usize {
        self.count
    }

    pub(super) fn push_object(&mut self, handle: HANDLE, event_id: EventID) -> Result<()> {
        if self.count == MAXIMUM_WAIT_OBJECTS as usize {
            return Err(Error::new_win32(ERROR_TOO_MANY_CMDS as _));
        }

        self.handles[self.count] = handle;
        self.event_ids[self.count] = Some(event_id);
        self.count += 1;

        Ok(())
    }

    /// Waits until an event in the list signals and returns the corresponding [`EventID`]
    pub(super) fn wait(&mut self, timeout: DWORD) -> Result<WaitResult> {
        let result = unsafe {
            WaitForMultipleObjectsEx(
                self.count as _,
                self.handles.as_mut_ptr(),
                FALSE,
                timeout,
                TRUE,
            )
        };

        if result == WAIT_FAILED {
            return Err(Error::get_last_error());
        } else if result == WAIT_IO_COMPLETION {
            return Ok(WaitResult::IOCompletion);
        } else if result == WAIT_TIMEOUT as _ || result as usize >= self.count {
            return Ok(WaitResult::Timeout);
        }

        let event_id = self.event_ids[result as usize];

        self.remove_index(result as usize);

        match event_id {
            Some(event_id) => Ok(WaitResult::Object(event_id)),
            None => Ok(WaitResult::IOCompletion),
        }
    }

    /// Removes an object from the list given its index
    fn remove_index(&mut self, index: usize) {
        self.count -= 1;

        if self.count == index {
            self.handles[index] = null_mut();
            self.event_ids[index] = None;
        } else {
            self.handles[index] = self.handles[self.count];
            self.event_ids[index] = self.event_ids[self.count];
        }
    }
}
