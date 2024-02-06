use crate::tasks::Task;
use std::{
    mem::ManuallyDrop,
    rc::Rc,
    task::{RawWaker, RawWakerVTable},
};

/// The [`RawWakerVTable`] for [`Task`]s
const RAW_WAKER_TABLE: RawWakerVTable =
    RawWakerVTable::new(clone_raw, wake_raw, wake_by_ref_raw, drop_raw);

/// Creates a [`RawWaker`] for a [`Task`]
pub(super) fn create_raw_waker(task: *const Task) -> RawWaker {
    RawWaker::new(task as _, &RAW_WAKER_TABLE)
}

/// Clones an [`Rc<Task>`] using its pointer.
///
/// `data` should have been created from either [`Rc::into_raw`] or [`Rc::as_ptr`]
unsafe fn clone_raw(data: *const ()) -> RawWaker {
    // Convert `data` into an `Rc<Task>`, manually drop to avoid decreasing the ref count
    let rc = ManuallyDrop::new(Rc::<Task>::from_raw(data as _));

    // Clone it to increase the ref count
    let _clone: ManuallyDrop<_> = rc.clone();

    // Return the waker
    create_raw_waker(data as _)
}

/// Calls `wake` on a [`Rc<Task>`] and consumes it
unsafe fn wake_raw(data: *const ()) {
    let rc = unsafe { Rc::<Task>::from_raw(data as _) };
    rc.wake()
}

/// Calls `wake` on a [`Rc<Task>`] without consuming it
unsafe fn wake_by_ref_raw(data: *const ()) {
    let rc = ManuallyDrop::new(unsafe { Rc::<Task>::from_raw(data as _) });
    rc.wake()
}

/// Drops an [`Rc<Task>`]
unsafe fn drop_raw(data: *const ()) {
    drop(unsafe { Rc::<Task>::from_raw(data as _) })
}
