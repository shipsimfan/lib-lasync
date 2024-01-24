//! Single-threaded executor for async/await

use task::Task;
use waker::WakerRef;

mod future_queue;
mod local_executor;
mod task;
mod waker;

pub use future_queue::FutureQueue;
pub use local_executor::LocalExecutor;
