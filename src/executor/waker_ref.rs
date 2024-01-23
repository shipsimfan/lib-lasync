use std::{marker::PhantomData, mem::ManuallyDrop, ops::Deref, task::Waker};

/// A reference to a [`Waker`]
pub(super) struct WakerRef<'a> {
    /// The [`Waker`] which this is "referencing"
    waker: ManuallyDrop<Waker>,

    /// A marker for the lifetime
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> WakerRef<'a> {
    /// Creates a new [`WakerRef`] from `waker`
    pub(super) fn new(waker: ManuallyDrop<Waker>) -> Self {
        WakerRef {
            waker,
            _lifetime: PhantomData,
        }
    }
}

impl<'a> Deref for WakerRef<'a> {
    type Target = Waker;

    fn deref(&self) -> &Self::Target {
        &self.waker
    }
}
