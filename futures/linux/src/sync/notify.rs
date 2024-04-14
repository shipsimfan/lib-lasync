use crate::event_ref::EventRef;
use executor::{
    platform::{
        linux::{
            linux::futex::{FUTEX_BITSET_MATCH_ANY, FUTEX_WAKE},
            sys::syscall::{syscall, SYS_futex},
            time::timespec,
            try_linux,
        },
        uring::{io_uring_cqe, io_uring_prep_futex_wait},
        EventHandler, WaitQueue,
    },
    EventID, EventManager, Result,
};
use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    ptr::{null, null_mut},
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
    task::{Context, Poll},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::sync::LocalNotify;

/// A [`Future`] which can be used to signal or be signalled by other tasks and threads
///
/// This can be signalled, but not waited upon, by other threads than the one that created it. If
/// this only needs to be used in one thread, consider using [`LocalNotify`].
pub struct Notify {
    /// Has this been notified. 1 indicates notified, 0 indicates not.
    ///
    /// This is used as a futex
    state: AtomicU32,

    /// The tasks to notify
    tasks: Rc<RefCell<WaitQueue>>,

    /// The event for registering the futex_wait I/O event
    event: EventRef,
}

/// A [`Future`] which yields when signalled by another task
pub struct Notified<'a> {
    /// The [`Notify`] to watch
    notify: &'a Notify,

    /// Has this [`Future`] been registered with the [`Notify`]?
    registered: bool,
}

/// Wakes the next task in the [`WaitQueue`]
fn notify_callback(_: &mut io_uring_cqe, tasks: &mut WaitQueue) {
    tasks.pop().map(|task| task.wake());
}

fn register_notify_event(event: EventID, futex: *mut u32) {
    EventManager::get_local_mut(|manager| {
        let sqe = manager.get_sqe(event).unwrap();

        unsafe {
            io_uring_prep_futex_wait(sqe.as_ptr(), futex, 0, FUTEX_BITSET_MATCH_ANY, 0, 0);
        }

        sqe.submit().unwrap();
    });
}

impl Notify {
    /// Creates a new unsignalled [`Notify`]
    pub fn new() -> Result<Self> {
        let tasks = Rc::new(RefCell::new(WaitQueue::new()));

        let event = EventRef::register(EventHandler::WaitQueue(tasks.clone(), notify_callback))?;

        Ok(Notify {
            state: AtomicU32::new(0),
            tasks,
            event,
        })
    }

    /// Notifies the next waiting task
    pub fn notify_one(&self) -> Result<()> {
        if self
            .state
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::Relaxed)
            .is_ok()
        {
            try_linux!(syscall(
                SYS_futex,
                self.state.as_ptr(),
                FUTEX_WAKE,
                1,
                null::<timespec>(),
                null_mut::<u32>(),
                0
            ))?;
        }

        Ok(())
    }

    /// Creates a [`Notified`] [`Future`] which can be `await`ed on to be signalled
    pub fn notified(&self) -> Notified {
        Notified {
            notify: self,
            registered: false,
        }
    }
}

unsafe impl Send for Notify {}
unsafe impl Sync for Notify {}

impl<'a> Future for Notified<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Is this the second time this poll is called?
        if self.registered {
            // If so, reset the state
            self.notify.state.store(0, Ordering::SeqCst);

            // Check to see if we need to re-register the `futex_wait` I/O event for future tasks
            if self.notify.tasks.borrow().len() > 0 {
                register_notify_event(*self.notify.event, self.notify.state.as_ptr());
            }

            return Poll::Ready(());
        }

        // Are there already tasks waiting?
        if self.notify.tasks.borrow().len() > 0 {
            // Place ourselves in the queue and wait
            self.notify.tasks.borrow_mut().push(cx.waker().clone());
            self.get_mut().registered = true;
            return Poll::Pending;
        }

        // Is the state currently signalled? (Nobody is waiting as checked above)
        if self
            .notify
            .state
            .compare_exchange(1, 0, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            // We took the signal
            return Poll::Ready(());
        }

        // Place ourselves in the queue and register the `futex_wait` I/O event
        self.notify.tasks.borrow_mut().push(cx.waker().clone());
        register_notify_event(*self.notify.event, self.notify.state.as_ptr());
        self.get_mut().registered = true;

        Poll::Pending
    }
}

impl<'a> !Send for Notified<'a> {}
impl<'a> !Sync for Notified<'a> {}
