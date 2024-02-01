use signal::SIGNAL_NUMBER;

mod id;
mod manager;
mod signal;
mod trigger;

pub use id::EventID;
pub use manager::EventManager;
pub use trigger::EventTrigger;
