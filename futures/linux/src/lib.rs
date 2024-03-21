//! Futures implemented for Linux

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(ip_bits)]

pub mod net;
pub mod time;

mod event_ref;

use event_ref::EventRef;
