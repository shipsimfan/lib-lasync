//! Platform specific definitions

#[cfg(target_os = "linux")]
pub use executor_linux::*;

#[cfg(target_os = "windows")]
pub use executor_windows::*;
