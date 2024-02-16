use std::{num::NonZeroUsize, time::Duration};

const SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };

#[test]
fn interval() {
    lasync::executor::run(SIZE, async {
        let mut interval = lasync::futures::time::interval(Duration::from_secs(1)).unwrap();

        let mut i = 0;
        while i < 6 {
            interval.tick().await;

            println!("Tick {}/5", i);

            i += 1;
        }
    })
    .unwrap();
}

#[test]
fn interval_with_delay() {
    lasync::executor::run(SIZE, async {
        let mut interval = lasync::futures::time::interval_with_delay(
            Duration::from_secs(2),
            Duration::from_secs(1),
        )
        .unwrap();

        let mut i = 1;
        while i < 6 {
            interval.tick().await;

            println!("Tick {}/5", i);

            i += 1;
        }
    })
    .unwrap();
}
