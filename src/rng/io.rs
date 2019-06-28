//! Random numbers from I/O.

use std::io;
use crate::prelude::*;

/// A wrapper around an
/// [`io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) for using the
/// type as a random number generator.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ReadRng<R: ?Sized>(pub R);

impl<R> From<R> for ReadRng<R> {
    #[inline]
    fn from(r: R) -> Self {
        ReadRng(r)
    }
}

impl<R: ?Sized> ReadRng<R> {
    /// Converts the mutable reference of the underlying type to a reference of
    /// type `Self`.
    #[inline]
    pub fn from_mut(r: &mut R) -> &mut Self {
        unsafe { &mut *(r as *mut R as *mut Self) }
    }
}

impl<R: ?Sized + io::Read> TryRng for ReadRng<R> {
    type Error = io::Error;

    #[inline]
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.0.read_exact(buf)
    }
}
