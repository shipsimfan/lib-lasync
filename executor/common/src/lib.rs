//! Common definitions for all platforms' executors

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod event;
mod event_id;
mod list;

pub use event::Event;
pub use event_id::EventID;
pub use list::List;
