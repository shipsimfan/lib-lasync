use std::{
    cell::RefCell,
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

/// A [`Future`] which can be used to signal or be signalled by other tasks
pub struct Notify(RefCell<NotifyInner>);

struct NotifyInner {
    /// Has this been notified
    state: bool,

    /// The tasks to notify
    tasks: VecDeque<Waker>,
}

/// A [`Future`] which yields when signalled by another task
pub struct Notified<'a> {
    /// The [`Notify`] to watch
    notify: &'a Notify,

    /// Has this [`Future`] been registered with the [`Notify`]?
    registered: bool,
}

impl Notify {
    /// Creates a new unsignalled [`Notify`]
    pub const fn new() -> Self {
        Notify(RefCell::new(NotifyInner {
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

    /// Creates a [`Notified`] [`Future`] which can be `await`ed on to be signalled
    pub fn notified(&self) -> Notified {
        Notified {
            notify: self,
            registered: false,
        }
    }
}

impl<'a> Future for Notified<'a> {
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
