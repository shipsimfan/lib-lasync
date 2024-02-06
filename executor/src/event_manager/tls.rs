use crate::platform::LocalEventManager;
use std::cell::RefCell;

thread_local! {
    /// The [`LocalEventManager`] for the current thread
    static LOCAL_EVENT_MANAGER: RefCell<Option<LocalEventManager>> = RefCell::new(None);
}

/// Gets the current thread's local event manager
pub(super) fn get_opt<T, F: FnOnce(&Option<LocalEventManager>) -> T>(f: F) -> T {
    LOCAL_EVENT_MANAGER.with(|manager| {
        let manager = manager.borrow();
        f(&*manager)
    })
}

/// Gets the current thread's local event manager mutably
pub(super) fn get_opt_mut<T, F: FnOnce(&mut Option<LocalEventManager>) -> T>(f: F) -> T {
    LOCAL_EVENT_MANAGER.with(|manager| {
        let mut manager = manager.borrow_mut();
        f(&mut *manager)
    })
}

/// Gets the current thread's local event manager
///
/// # Panic
/// This function will panic if the local event manager has not been set
pub(super) fn get<T, F: FnOnce(&LocalEventManager) -> T>(f: F) -> T {
    get_opt(|manager| f(manager.as_ref().unwrap()))
}

/// Gets the current thread's local event manager mutably
///
/// # Panic
/// This function will panic if the local event manager has not been set
pub(super) fn get_mut<T, F: FnOnce(&LocalEventManager) -> T>(f: F) -> T {
    get_opt_mut(|manager| f(manager.as_mut().unwrap()))
}
