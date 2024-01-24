use crate::executor::Task;
use std::{
    mem::ManuallyDrop,
    rc::Rc,
    task::{RawWaker, RawWakerVTable},
};

/// Creates a [`RawWaker`] for a [`Task`]
pub(super) fn create_raw_waker<T>(task: *const Task<T>) -> RawWaker {
    RawWaker::new(task as _, raw_waker_vtable::<T>())
}

/// The [`RawWakerVTable`] for [`Task`]s
const fn raw_waker_vtable<T>() -> &'static RawWakerVTable {
    &RawWakerVTable::new(
        clone_raw::<T>,
        wake_raw::<T>,
        wake_by_ref_raw::<T>,
        drop_raw::<T>,
    )
}

/// Clones an [`Rc<Task>`] using its pointer.
///
/// `data` should have been created from either [`Rc::into_raw`] or [`Rc::as_ptr`]
unsafe fn clone_raw<T>(data: *const ()) -> RawWaker {
    // Convert `data` into an `Rc<Task>`, manually drop to avoid decreasing the ref count
    let rc = ManuallyDrop::new(Rc::<Task<T>>::from_raw(data as _));

    // Clone it to increase the ref count
    let _clone: ManuallyDrop<_> = rc.clone();

    // Return the waker
    create_raw_waker::<T>(data as _)
}

/// Calls `wake` on a [`Rc<Task>`] and consumes it
unsafe fn wake_raw<T>(data: *const ()) {
    let rc = unsafe { Rc::<Task<T>>::from_raw(data as _) };
    rc.wake()
}

/// Calls `wake` on a [`Rc<Task>`] without consuming it
unsafe fn wake_by_ref_raw<T>(data: *const ()) {
    let rc = ManuallyDrop::new(unsafe { Rc::<Task<T>>::from_raw(data as _) });
    rc.wake()
}

/// Drops an [`Rc<Task>`]
unsafe fn drop_raw<T>(data: *const ()) {
    drop(unsafe { Rc::<Task<T>>::from_raw(data as _) })
}
