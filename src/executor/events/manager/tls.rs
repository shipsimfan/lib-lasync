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

    pub(super) fn register(new_manager: LocalEventManager) -> () {
        let mut slot = manager_slot.borrow_mut();

        assert!(slot.is_none(), "attempting to register an event manager while one is already registered on the current thread");

        *slot = Some(new_manager);
    }

    pub(super) fn unregister() -> () {
        *manager_slot.borrow_mut() = None;
    }

    pub(super) fn len() -> usize {
        let manager = manager_slot.borrow();
        manager.as_ref().unwrap().len()
    }
];
