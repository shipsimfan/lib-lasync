//! Futures implemented for Linux

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(negative_impls)]

pub mod fs;
pub mod io;
pub mod net;
pub mod sync;
pub mod time;

mod event_ref;
mod fd;

use event_ref::EventRef;
use fd::{AsFD, FDRead};
