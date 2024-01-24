use super::task::Task;
use std::{marker::PhantomData, mem::ManuallyDrop, ops::Deref, rc::Rc, task::Waker};

mod vtable;

/// A reference to a [`Waker`]
pub(super) struct WakerRef<'a, T> {
    /// The [`Waker`] which this is "referencing"
    waker: ManuallyDrop<Waker>,

    /// A marker for the lifetime
    _lifetime: PhantomData<&'a T>,
}

/// Creates a [`Waker`] for a [`Task`]
fn create_waker<T>(task: *const Task<T>) -> Waker {
    unsafe { Waker::from_raw(vtable::create_raw_waker(task)) }
}

impl<'a, T> WakerRef<'a, T> {
    /// Creates a new [`WakerRef`] for a [`Task`]
    pub(super) fn new(task: &'a Rc<Task<T>>) -> Self {
        let waker = ManuallyDrop::new(create_waker(Rc::as_ptr(task)));

        WakerRef {
            waker,
            _lifetime: PhantomData,
        }
    }
}

impl<'a, T> Deref for WakerRef<'a, T> {
    type Target = Waker;

    fn deref(&self) -> &Self::Target {
        &self.waker
    }
}
