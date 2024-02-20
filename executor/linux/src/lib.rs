//! Executor implementation for Linux

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod event_handler;
mod io_uring;
mod manager;

pub use event_handler::EventHandler;
pub use manager::LocalEventManager;

// The result used for Linux events
pub use linux::{Error, Result};

use io_uring::IOURing;
