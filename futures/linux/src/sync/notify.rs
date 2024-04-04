use executor::platform::WaitQueue;
use std::sync::atomic::AtomicU32;

// rustdoc imports
#[allow(unused_imports)]
use crate::sync::LocalNotify;

/// A [`Future`] which can be used to signal or be signalled by other tasks and threads
///
/// This can be signalled, but not waited upon, by other threads than the one that created it. If
/// this only needs to be used in one thread, consider using [`LocalNotify`].
pub struct Notify {
    /// Has this been notified
    ///
    /// This is used as a futex
    state: AtomicU32,

    /// The tasks to notify
    tasks: WaitQueue,
}

unsafe impl Send for Notify {}
unsafe impl Sync for Notify {}
