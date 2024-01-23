use std::time::Duration;

use lasync::{executor::LocalExecutor, futures::Timer};

#[test]
fn simple() {
    let (exeuctor, queue) = LocalExecutor::new();

    queue.push(async {
        println!("Hello");
        Timer::new(Duration::from_secs(1)).await;
        println!("World!");
    });

    drop(queue);

    exeuctor.run();
}
