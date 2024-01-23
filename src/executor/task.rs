use super::{FutureQueue, WakerRef};
use std::{
    future::Future,
    mem::ManuallyDrop,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{RawWaker, RawWakerVTable, Waker},
};

/// A [`Future`] which can re-schedule itself
pub(super) struct Task {
    /// The [`Future`] to be driven to completion
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>,

    /// The queue to re-schedule the future on
    future_queue: FutureQueue,
}

impl Task {
    const RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
        Self::clone_raw,
        Self::wake_raw,
        Self::wake_by_ref_raw,
        Self::drop_raw,
    );

    /// Creates a new [`Task`] for `future`
    pub(super) fn new(
        future: impl Future<Output = ()> + Send + 'static,
        future_queue: FutureQueue,
    ) -> Self {
        Task {
            future: Mutex::new(Some(Box::pin(future))),
            future_queue,
        }
    }

    pub(super) fn future(
        &self,
    ) -> &Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>> {
        &self.future
    }

    pub(super) fn waker<'a>(self: &'a Arc<Self>) -> WakerRef<'a> {
        let ptr = Arc::as_ptr(self) as _;

        let waker = ManuallyDrop::new(unsafe { Waker::from_raw(Self::raw_waker(ptr)) });
        WakerRef::new(waker)
    }

    fn wake(self: &Arc<Self>) {
        let cloned = self.clone();
        self.future_queue.push_raw(cloned);
    }

    const unsafe fn raw_waker(data: *const ()) -> RawWaker {
        RawWaker::new(data, &Self::RAW_WAKER_VTABLE)
    }

    unsafe fn clone_raw(data: *const ()) -> RawWaker {
        let arc = ManuallyDrop::new(Arc::<Self>::from_raw(data as _));
        let _clone: ManuallyDrop<_> = arc.clone();
        Self::raw_waker(data)
    }

    unsafe fn wake_raw(data: *const ()) {
        let arc = unsafe { Arc::<Self>::from_raw(data as _) };
        arc.wake()
    }

    unsafe fn wake_by_ref_raw(data: *const ()) {
        let arc = ManuallyDrop::new(unsafe { Arc::<Self>::from_raw(data as _) });
        arc.wake()
    }

    unsafe fn drop_raw(data: *const ()) {
        drop(unsafe { Arc::<Self>::from_raw(data as _) })
    }
}
