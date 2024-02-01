use super::LocalEventManager;
use std::cell::RefCell;

thread_local! {
    /// The [`LocalEventManager`] for the current thread
    static LOCAL_EVENT_MANAGER: RefCell<Option<LocalEventManager>> = RefCell::new(None);
}

macro_rules! local_thread_functions {
    [
        static $local_event_manager: ident: &RefCell<Option<LocalEventManager>>;

        $(
            $(#[$attrib:meta])*
            pub(super) fn $name: ident (
                $($parameter_name: ident: $parameter_type: ty),*
            ) -> $return: ty $body: block
        )*
    ] => {
        $(
            $(#[$attrib])*
            pub(super) fn $name($($parameter_name: $parameter_type),*) -> $return {
                LOCAL_EVENT_MANAGER.with(|$local_event_manager: &RefCell<Option<LocalEventManager>>| $body)
            }
        )*
    };
}

local_thread_functions![
    static manager_slot: &RefCell<Option<LocalEventManager>>;

    /// Registers a [`LocalEventManager`] as the current thread's event manager
    pub(super) fn register(new_manager: LocalEventManager) -> () {
        let mut slot = manager_slot.borrow_mut();

        assert!(slot.is_none(), "attempting to register an event manager while one is already registered on the current thread");

        *slot = Some(new_manager);
    }

    /// Unregisters the current thread's event manager
    pub(super) fn unregister() -> () {
        *manager_slot.borrow_mut() = None;
    }
];

/// Gets the current thread's local event manager
pub(super) fn get<T, F: FnOnce(&LocalEventManager) -> T>(f: F) -> T {
    LOCAL_EVENT_MANAGER.with(|manager| {
        let manager = manager.borrow();
        let manager = manager.as_ref().unwrap();
        f(manager)
    })
}

/// Gets the current thread's local event manager mutably
pub(super) fn get_mut<T, F: FnOnce(&mut LocalEventManager) -> T>(f: F) -> T {
    LOCAL_EVENT_MANAGER.with(|manager| {
        let mut manager = manager.borrow_mut();
        let manager = manager.as_mut().unwrap();
        f(manager)
    })
}
