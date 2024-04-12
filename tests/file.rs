use lasync::fs::OpenOptions;
use std::num::NonZeroUsize;

const SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };
const READ_PATH: &str = "./tests/test_data.txt";

#[test]
fn file_read() {
    lasync::run(SIZE, async {
        let file = OpenOptions::new().read(true).open(READ_PATH).await.unwrap();

        drop(file);
    })
    .unwrap();
}
