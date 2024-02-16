//! Futures implemented for Windows

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod event_ref;

pub mod time;

pub use win32::{Error, Result};

use event_ref::EventRef;
