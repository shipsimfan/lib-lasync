use std::{
    num::NonZeroUsize,
    time::{Duration, Instant},
};

const SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };

#[test]
fn one_timer() {
    let start = Instant::now();

    lasync::executor::run(SIZE, async {
        println!("Hello");

        lasync::futures::time::sleep(Duration::from_secs(1))
            .unwrap()
            .await;

        println!("World!");
    })
    .unwrap();

    let end = Instant::now();
    let duration = end.duration_since(start);
    assert!(duration.as_secs_f64() >= 1.);
}

#[test]
fn two_timers() {
    let queue = lasync::executor::FutureQueue::new();

    queue.push(async {
        println!("Task 1 - Start");

        lasync::futures::time::sleep(Duration::from_millis(1500))
            .unwrap()
            .await;

        println!("Task 1 - End");
    });

    queue.push(async {
        println!("Task 2 - Start");

        lasync::futures::time::sleep(Duration::from_millis(500))
            .unwrap()
            .await;

        println!("Task 2 - Middle");

        lasync::futures::time::sleep(Duration::from_millis(1500))
            .unwrap()
            .await;

        println!("Task 2 - End");
    });

    let start = Instant::now();

    lasync::executor::run_queue(SIZE, queue).unwrap();

    let end = Instant::now();
    let duration = end.duration_since(start);
    assert!(duration.as_secs_f64() >= 2.);
}
