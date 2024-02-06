//! A single-threaded async executor
//!
//! An async executor for the current thread can be started using either the [`executor::run`]
//! function to run a single [`Future`] or the [`executor::run_queue`] function to run multiple
//! [`Future`]s. The executor will drive all [`Future`]s given to it to completion and then return.
//!
//! A [`FutureQueue`] can be [`Clone`]d and the [`Clone`]d [`executor::FutureQueue`] will point to
//! the same underlying queue. This allows more [`Future`]s to be given to an executor during
//! execution. [`executor::FutureQueue`]s are `!Send + !Sync` so they cannot be safely used from a
//! different thread.

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

pub use executor;
pub use futures;

// rustdoc imports
#[allow(unused_imports)]
use std::future::Future;
