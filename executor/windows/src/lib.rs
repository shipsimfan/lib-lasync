//! Executor implementation for Windows

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod manager;

pub use manager::LocalEventManager;

/// TODO: Temporary placeholder, replace with [`win32::Result`]
pub type Result<T> = std::io::Result<T>;
