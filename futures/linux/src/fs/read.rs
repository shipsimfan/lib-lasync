use crate::{fs::File, io::Read};
use executor::{Error, Result};
use std::path::Path;

/// Reads a file into a buffer
pub async fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut file = File::open(path).await?;
    let size = file
        .metadata()
        .await
        .map(|metadata| metadata.len())
        .unwrap_or(0) as usize;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size).map_err(|_| Error::ENOMEM)?;

    let buffer = unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr(), size) };
    file.read_exact(buffer).await?;
    unsafe { bytes.set_len(size) };

    Ok(bytes)
}
