use std::{
    cell::RefCell,
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

/// A [`Future`] which can be used to signal or be signalled by other tasks
///
/// This can only be used on one thread. For signalling across threads, see [`Notify`].
pub struct LocalNotify(RefCell<LocalNotifyInner>);

struct LocalNotifyInner {
    /// Has this been notified
    state: bool,

    /// The tasks to notify
    tasks: VecDeque<Waker>,
}

/// A [`Future`] which yields when signalled by another task
pub struct LocalNotified<'a> {
    /// The [`LocalNotify`] to watch
    notify: &'a LocalNotify,

    /// Has this [`Future`] been registered with the [`LocalNotify`]?
    registered: bool,
}

impl LocalNotify {
    /// Creates a new unsignalled [`LocalNotify`]
    pub const fn new() -> Self {
        LocalNotify(RefCell::new(LocalNotifyInner {
            state: false,
            tasks: VecDeque::new(),
        }))
    }

    /// Notifies the next waiting task
    pub fn notify_one(&self) {
        let mut notify = self.0.borrow_mut();

        if let Some(task) = notify.tasks.pop_front() {
            task.wake();
            notify.state = false;
        } else {
            notify.state = true;
        }
    }

    /// Notifies all currently waiting tasks
    pub fn notify_all(&self) {
        let mut notify = self.0.borrow_mut();

        while let Some(task) = notify.tasks.pop_front() {
            task.wake();
        }

        notify.state = false;
    }

    /// Creates a [`LocalNotified`] [`Future`] which can be `await`ed on to be signalled
    pub fn notified(&self) -> LocalNotified {
        LocalNotified {
            notify: self,
            registered: false,
        }
    }
}

impl !Send for LocalNotify {}
impl !Sync for LocalNotify {}

impl<'a> Future for LocalNotified<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.registered {
            Poll::Ready(())
        } else {
            let this = self.get_mut();
            this.registered = true;

            let mut notify = this.notify.0.borrow_mut();
            if notify.state {
                assert_eq!(notify.tasks.len(), 0);

                notify.state = false;
                return Poll::Ready(());
            }

            notify.tasks.push_back(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl<'a> !Send for LocalNotified<'a> {}
impl<'a> !Sync for LocalNotified<'a> {}
