use std::{
    num::NonZeroUsize,
    time::{Duration, Instant},
};

const SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };

fn run_interval(count: usize, tick: Duration) {
    let target_duration = count as f64 * tick.as_secs_f64();

    let start = Instant::now();

    lasync::executor::run(SIZE, async move {
        let mut interval = lasync::futures::time::interval(tick).unwrap();

        for i in 0..count {
            interval.tick().await;

            println!("Tick {}/{count}", i + 1);
        }
    })
    .unwrap();

    let end = Instant::now();
    let duration = end.duration_since(start);
    assert!(duration.as_secs_f64() >= target_duration);
}

#[test]
fn interval() {
    run_interval(5, Duration::from_secs(1))
}
