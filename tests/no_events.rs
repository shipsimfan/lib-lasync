use executor::FutureQueue;
use std::num::NonZeroUsize;

const SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };

#[test]
fn no_events() {
    lasync::executor::run_queue(SIZE, FutureQueue::new()).unwrap();
}
