//! Single-threaded executor for async/await

use task::Task;
use waker_ref::WakerRef;

mod future_queue;
mod local_executor;
mod task;
mod waker_ref;

pub use future_queue::FutureQueue;
pub use local_executor::LocalExecutor;
