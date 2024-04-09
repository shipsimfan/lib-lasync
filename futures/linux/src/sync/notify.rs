use executor::platform::WaitQueue;
use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicU32};
use uring::io_uring_cqe;

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
    tasks: Rc<RefCell<WaitQueue>>,
}

/// Wakes the next task in the [`WaitQueue`]
fn notify_callback(_: &mut io_uring_cqe, tasks: &mut WaitQueue) {
    tasks.pop().map(|task| task.wake());
}

impl Notify {
    // `state` == 0 means not notified
    // `state` == 1 means notified
    //
    // To wait:
    //  1. Check to see if there are tasks in the queue
    //    1a. If there are, put yourself in the queue and return Poll::Pending
    //  2. Compare the state with 1 and exchange it to 0 if it is 1
    //    2a. If the state was 1, the state is now set to 0 and you took the signal, return
    //        Poll::Ready.
    //  3. If it was 0, add yourself to the queue, register the `futex_wait`/notify callback if
    //     needed, and return Poll::Pending.
    //
    // If Poll::Pending was returned, the next poll will happen after the task is woken by the
    // notify callback. In the second poll:
    //  1. Atomically store 0 in the state.
    //  2. Re-register the `futex_wait` if there are more tasks waiting (it is no longer in the
    //     io_uring as that is what waked this task)
    //  3. Return Poll::Ready
    //
    // To notify:
    //  1. Atomically store 1
    //  2. Call futex_wake
    //
    // Callback:
    //  1. Wake the next task in the queue
}

unsafe impl Send for Notify {}
unsafe impl Sync for Notify {}
