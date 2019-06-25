//! Generate random numbers.

use core::convert::Infallible;

#[cfg(feature = "std")]
pub mod io;

/// A type that can be used to generate random numbers without fail.
pub trait Rng {
    /// Fills `buf` with random bytes from `self`.
    fn fill_bytes(&mut self, buf: &mut [u8]);

    /// Generates a random 8-bit integer.
    #[inline]
    fn next_u8(&mut self) -> u8 {
        let mut byte = 0;
        self.fill_bytes(core::slice::from_mut(&mut byte));
        byte
    }

    /// Generates a random 16-bit integer.
    #[inline]
    fn next_u16(&mut self) -> u16 {
        let mut bytes = [0; 2];
        self.fill_bytes(&mut bytes);
        u16::from_ne_bytes(bytes)
    }

    /// Generates a random 32-bit integer.
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0; 4];
        self.fill_bytes(&mut bytes);
        u32::from_ne_bytes(bytes)
    }

    /// Generates a random 64-bit integer.
    #[inline]
    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0; 8];
        self.fill_bytes(&mut bytes);
        u64::from_ne_bytes(bytes)
    }

    /// Generates a random 128-bit integer.
    #[inline]
    fn next_u128(&mut self) -> u128 {
        let mut bytes = [0; 16];
        self.fill_bytes(&mut bytes);
        u128::from_ne_bytes(bytes)
    }
}

/// A type that can be used to generate random numbers, and may fail to do so.
pub trait TryRng {
    /// The error reported when `Self` fails to generate random numbers.
    type Error;

    /// Fills `buf` with random bytes from `self`, returning an error upon
    /// failure.
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;

    /// Generates a random 8-bit integer, returning an error upon failure.
    #[inline]
    fn try_next_u8(&mut self) -> Result<u8, Self::Error> {
        let mut byte = 0;
        self.try_fill_bytes(core::slice::from_mut(&mut byte))?;
        Ok(byte)
    }

    /// Generates a random 16-bit integer, returning an error upon failure.
    #[inline]
    fn try_next_u16(&mut self) -> Result<u16, Self::Error> {
        let mut bytes = [0; 2];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u16::from_ne_bytes(bytes))
    }

    /// Generates a random 32-bit integer, returning an error upon failure.
    #[inline]
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut bytes = [0; 4];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u32::from_ne_bytes(bytes))
    }

    /// Generates a random 64-bit integer, returning an error upon failure.
    #[inline]
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut bytes = [0; 8];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u64::from_ne_bytes(bytes))
    }

    /// Generates a random 128-bit integer, returning an error upon failure.
    #[inline]
    fn try_next_u128(&mut self) -> Result<u128, Self::Error> {
        let mut bytes = [0; 16];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u128::from_ne_bytes(bytes))
    }
}

impl<R: ?Sized + Rng> TryRng for R {
    type Error = Infallible;

    #[inline]
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        Ok(self.fill_bytes(buf))
    }

    #[inline]
    fn try_next_u8(&mut self) -> Result<u8, Self::Error> {
        Ok(self.next_u8())
    }

    #[inline]
    fn try_next_u16(&mut self) -> Result<u16, Self::Error> {
        Ok(self.next_u16())
    }

    #[inline]
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.next_u32())
    }

    #[inline]
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.next_u64())
    }

    #[inline]
    fn try_next_u128(&mut self) -> Result<u128, Self::Error> {
        Ok(self.next_u128())
    }
}

// TODO: Figure out how to do the following without conflicting:
// impl<R: ?Sized + TryRng> TryRng for &mut R
