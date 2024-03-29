//! Executor implementation for Windows

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod manager;
mod objects;

pub use manager::LocalEventManager;

pub use win32::{Error, Result};

use objects::{Objects, WaitResult};
