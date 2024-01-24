use crate::executor::Task;
use std::{
    mem::ManuallyDrop,
    sync::Arc,
    task::{RawWaker, RawWakerVTable},
};

/// The [`RawWakerVTable`] for [`Task`]s
const RAW_WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(clone_raw, wake_raw, wake_by_ref_raw, drop_raw);

/// Creates a [`RawWaker`] for a [`Task`]
pub(super) fn create_raw_waker(task: *const Task) -> RawWaker {
    RawWaker::new(task as _, &RAW_WAKER_VTABLE)
}

/// Clones an [`Arc<Task>`] using its pointer.
///
/// `data` should have been created from either [`Arc::into_raw`] or [`Arc::as_ptr`]
unsafe fn clone_raw(data: *const ()) -> RawWaker {
    // Convert `data` into an `Arc<Task>`, manually drop to avoid decreasing the ref count
    let arc = ManuallyDrop::new(Arc::<Task>::from_raw(data as _));

    // Clone it to increase the ref count
    let _clone: ManuallyDrop<_> = arc.clone();

    // Return the waker
    create_raw_waker(data as _)
}

/// Calls `wake` on a [`Arc<Task>`] and consumes it
unsafe fn wake_raw(data: *const ()) {
    let arc = unsafe { Arc::<Task>::from_raw(data as _) };
    arc.wake()
}

/// Calls `wake` on a [`Arc<Task>`] without consuming it
unsafe fn wake_by_ref_raw(data: *const ()) {
    let arc = ManuallyDrop::new(unsafe { Arc::<Task>::from_raw(data as _) });
    arc.wake()
}

/// Drops an [`Arc<Task>`]
unsafe fn drop_raw(data: *const ()) {
    drop(unsafe { Arc::<Task>::from_raw(data as _) })
}
