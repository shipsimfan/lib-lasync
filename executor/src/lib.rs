//! The executor which runs [`Future`]s

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod event_manager;
mod platform;
mod run;
mod tasks;

pub use event_manager::EventManager;
pub use executor_common::EventID;
pub use run::{run, run_queue};
pub use tasks::FutureQueue;

use tasks::WakerRef;
