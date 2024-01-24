use lasync::futures::Timer;
use std::time::Duration;

#[test]
fn simple() {
    lasync::executor::run(async {
        println!("Hello");
        Timer::new(Duration::from_secs(1)).unwrap().await;
        println!("World!");
    });
}
