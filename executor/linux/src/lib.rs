//! Executor implementation for Linux

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod manager;

pub use manager::LocalEventManager;

/// The result used for Linux events
pub type Result<T> = linux::Result<T>;