/*use std::{ffi::c_int, time::Duration};

const SIGNAL_NUMBER: c_int = linux::signal::SIGUSR1;

#[test]
fn one_timer() {
    lasync::executor::run(SIGNAL_NUMBER, async {
        println!("Hello");

        lasync::futures::Timer::new(Duration::from_secs(1))
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

        lasync::futures::Timer::new(Duration::from_millis(1500))
            .unwrap()
            .await;

        println!("Task 1 - End");
    });

    queue.push(async {
        println!("Task 2 - Start");

        lasync::futures::Timer::new(Duration::from_millis(500))
            .unwrap()
            .await;

        println!("Task 2 - Middle");

        lasync::futures::Timer::new(Duration::from_millis(1500))
            .unwrap()
            .await;

        println!("Task 2 - End");
    });

    lasync::executor::run_queue(SIGNAL_NUMBER, queue).unwrap();
}
*/