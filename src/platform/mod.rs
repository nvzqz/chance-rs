//! Platform-specific functionality.

#[cfg(target_vendor = "apple")]
pub mod apple;

#[cfg(unix)]
pub mod unix;
