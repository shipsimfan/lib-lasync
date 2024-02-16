use std::{num::NonZeroUsize, time::Duration};

const SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };

#[test]
fn one_timer() {
    lasync::executor::run(SIZE, async {
        println!("Hello");

        lasync::futures::time::sleep(Duration::from_secs(1))
            .unwrap()
            .await;

        println!("World!");
    })
    .unwrap();
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

    lasync::executor::run_queue(SIZE, queue).unwrap();
}