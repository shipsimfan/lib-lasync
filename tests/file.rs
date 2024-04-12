use lasync::{fs::OpenOptions, io::Read};
use std::num::NonZeroUsize;

const SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };
const READ_PATH: &str = "./tests/test_data.txt";

const TEST_CONTENT: &[u8] = include_bytes!("./test_data.txt");

#[test]
fn file_read() {
    lasync::run(SIZE, async {
        let mut file = OpenOptions::new().read(true).open(READ_PATH).await.unwrap();

        let mut buffer = [0; TEST_CONTENT.len()];
        file.read_exact(&mut buffer).await.unwrap();

        assert_eq!(buffer, TEST_CONTENT);

        println!("{}", String::from_utf8_lossy(&buffer));
    })
    .unwrap();
}
