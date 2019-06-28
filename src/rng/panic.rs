use core::fmt::Debug;
use crate::prelude::*;

/// A wrapper around a [`TryRng`](trait.TryRng.html) that implements
/// [`Rng`](trait.Rng.html) via panicking if an error occurred.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PanickingRng<R: ?Sized>(pub R);

impl<R: ?Sized> PanickingRng<R> {
    /// Wraps the mutable reference `rng` as a mutable reference of type
    /// `PanickingRng`.
    #[inline]
    pub fn from_ref(rng: &mut R) -> &mut Self {
        unsafe { &mut *(rng as *mut R as *mut Self) }
    }
}

impl<R: ?Sized + TryRng> Rng for PanickingRng<R>
    where R::Error: Debug
{
    #[inline]
    fn fill_bytes(&mut self, buf: &mut [u8]) {
        self.0.try_fill_bytes(buf).unwrap();
    }

    #[inline]
    fn next_u8(&mut self) -> u8 {
        self.0.try_next_u8().unwrap()
    }

    #[inline]
    fn next_u16(&mut self) -> u16 {
        self.0.try_next_u16().unwrap()
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.0.try_next_u32().unwrap()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.0.try_next_u64().unwrap()
    }

    #[inline]
    fn next_u128(&mut self) -> u128 {
        self.0.try_next_u128().unwrap()
    }
}
