//! The futures used by lasync

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

pub use futures_common::*;

#[cfg(target_os = "windows")]
pub use futures_windows::*;

#[cfg(target_os = "linux")]
pub use futures_linux::*;
