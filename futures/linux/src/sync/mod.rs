//! Futures for synchronization

mod local;
mod notify;

pub use local::{LocalNotified, LocalNotify};
pub use notify::Notify;
