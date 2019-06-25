//! Generate random values.

use crate::{Rng, TryRng};

/// A type that can be generated from an `Rng`.
pub trait Rand: Sized {
    /// Generates a random instance of `self`, returning an error upon failure.
    fn rand<R: ?Sized + Rng>(rng: &mut R) -> Self;

    /// Generates a random instance of `self` in-place, returning an error upon
    /// failure.
    #[inline]
    fn read_rand<R: ?Sized + Rng>(&mut self, rng: &mut R) {
        *self = Self::rand(rng)
    }
}

impl Rand for u8 {
    #[inline]
    fn rand<R: ?Sized + Rng>(rng: &mut R) -> Self {
        rng.next_u8()
    }
}

impl Rand for u16 {
    #[inline]
    fn rand<R: ?Sized + Rng>(rng: &mut R) -> Self {
        rng.next_u16()
    }
}

impl Rand for u32 {
    #[inline]
    fn rand<R: ?Sized + Rng>(rng: &mut R) -> Self {
        rng.next_u32()
    }
}

impl Rand for u64 {
    #[inline]
    fn rand<R: ?Sized + Rng>(rng: &mut R) -> Self {
        rng.next_u64()
    }
}

impl Rand for u128 {
    #[inline]
    fn rand<R: ?Sized + Rng>(rng: &mut R) -> Self {
        rng.next_u128()
    }
}

impl Rand for usize {
    #[inline]
    fn rand<R: ?Sized + Rng>(rng: &mut R) -> Self {
        #[cfg(target_pointer_width = "16")]
        { rng.next_u16() as _ }

        #[cfg(target_pointer_width = "32")]
        { rng.next_u32() as _ }

        #[cfg(target_pointer_width = "64")]
        { rng.next_u64() as _ }
    }
}

/// A type that can be generated from a `TryRng`.
pub trait TryRand: Sized {
    /// Generates a random instance of `self`, returning an error upon failure.
    fn try_rand<R: ?Sized + TryRng>(rng: &mut R) -> Result<Self, R::Error>;

    /// Generates a random instance of `self` in-place, returning an error upon
    /// failure.
    #[inline]
    fn try_read_rand<R: ?Sized + TryRng>(&mut self, rng: &mut R) -> Result<(), R::Error> {
        *self = Self::try_rand(rng)?;
        Ok(())
    }
}

impl TryRand for u8 {
    #[inline]
    fn try_rand<R: ?Sized + TryRng>(rng: &mut R) -> Result<Self, R::Error> {
        rng.try_next_u8()
    }
}

impl TryRand for u16 {
    #[inline]
    fn try_rand<R: ?Sized + TryRng>(rng: &mut R) -> Result<Self, R::Error> {
        rng.try_next_u16()
    }
}

impl TryRand for u32 {
    #[inline]
    fn try_rand<R: ?Sized + TryRng>(rng: &mut R) -> Result<Self, R::Error> {
        rng.try_next_u32()
    }
}

impl TryRand for u64 {
    #[inline]
    fn try_rand<R: ?Sized + TryRng>(rng: &mut R) -> Result<Self, R::Error> {
        rng.try_next_u64()
    }
}

impl TryRand for u128 {
    #[inline]
    fn try_rand<R: ?Sized + TryRng>(rng: &mut R) -> Result<Self, R::Error> {
        rng.try_next_u128()
    }
}
