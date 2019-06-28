#![allow(non_camel_case_types, non_upper_case_globals)]

use core::{ffi::c_void, ptr};
use crate::prelude::*;

type c_int = i32;

const kSecRandomDefault: *const c_void = ptr::null();
const errSecSuccess: c_int = 0;

#[link(name = "Security", kind = "framework")]
extern {
    fn SecRandomCopyBytes(rng: *const c_void, count: isize, bytes: *mut u8) -> c_int;
}

/// A cryptographically secure random number generator that uses
/// `SecRandomCopyBytes` found in `Security.framework`.
///
/// # Availability
///
/// - iOS 2.0+
/// - macOS 10.7+
/// - tvOS 9.0+
/// - watchOS 2.0+
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecRandom {
    sec_random_ref: *const c_void
}

impl Default for SecRandom {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl TryRng for SecRandom {
    type Error = SecRandomError;

    #[inline]
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        (&*self).try_fill_bytes(buf)
    }
}

// OK because `SecRandomCopyBytes` takes a `const` pointer
impl TryRng for &SecRandom {
    type Error = SecRandomError;

    #[inline]
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        let len = buf.len() as isize;
        let ptr = buf.as_mut_ptr();
        match unsafe { SecRandomCopyBytes(self.sec_random_ref, len, ptr) } {
            errSecSuccess => Ok(()),
            err => Err(SecRandomError(err)),
        }
    }
}

impl CryptoRng for SecRandom {}

impl SecRandom {
    /// Creates a new instance for the default `SecRandomRef`.
    #[inline]
    pub const fn new() -> Self {
        SecRandom { sec_random_ref: kSecRandomDefault }
    }

    /// Creates a new instance for a `SecRandomRef`.
    #[inline]
    pub const unsafe fn from_ref(sec_random_ref: *const c_void) -> Self {
        SecRandom { sec_random_ref }
    }
}

/// An error returned when [`SecRandom`](struct.SecRandom.html) fails.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecRandomError(i32);
