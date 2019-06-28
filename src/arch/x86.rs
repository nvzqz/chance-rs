//! x86 family specific functionality.
//!
//! This module contains two random number generators: `rdrand` and `rdsweed`.
//! The differences are outlined
//! [here](https://software.intel.com/en-us/blogs/2012/11/17/the-difference-between-rdrand-and-rdseed).

#[cfg(target_arch = "x86")]
use core::arch::x86::{
    _rdrand16_step,
    _rdrand32_step,
    _rdseed16_step,
    _rdseed32_step,
};

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    _rdrand16_step,
    _rdrand32_step,
    _rdrand64_step,
    _rdseed16_step,
    _rdseed32_step,
    _rdseed64_step,
};

use crate::prelude::*;

/// A random number generator that uses the `rdrand` instruction.
///
/// It is considered to be a cryptographically secure pseudorandom number
/// generator.
///
/// Taken from [here](https://software.intel.com/en-us/blogs/2012/11/17/the-difference-between-rdrand-and-rdseed):
///
/// > In contrast \[to RDSEED\], RDRAND is the output of a 128-bit PRNG that is
/// > compliant to NIST SP 800-90A. It is intended for applications that simply
/// > need high-quality random numbers. The numbers returned by RDRAND have
/// > additive prediction resistance because they are the output of a pseurandom
/// > number generator. If you put two 64-bit values with additive prediction
/// > resistance togehter, the prediction resistance of the resulting value is
/// > only 65 bits (264 + 264 = 265). To ensure that RDRAND values are fully
/// > prediction-resistant when combined together to build larger values you can
/// > follow the procedures in the DRNG Software Implementation Guide on
/// > generating seed values from RDRAND, but it's generally best and simplest
/// > to just use RDSEED for PRNG seeding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RdRand(());

/// A random number generator that uses the `rdseed` instruction.
///
/// It is considered to be a non-deterministic random bit generator.
///
/// Taken from [here](https://software.intel.com/en-us/blogs/2012/11/17/the-difference-between-rdrand-and-rdseed):
///
/// > The numbers returned by RDSEED are referred to as "seed-grade entropy" and
/// > are the output of a true random number generator (TRNG), or an ehanced
/// > non-deterministic random number generator (ENRNG) in NIST-speak.  RDSEED
/// > is intended for use by software vendors who have an existing PRNG, but
/// > would like to benefit from the entropy source of Intel Secure Key. With
/// > RDSEED you can seed a PRNG of any size.
/// >
/// > The numbers retuned by RDSEED have multiplicative prediction resistance.
/// > If you use two 64-bit samples with multiplicative prediction resistance to
/// > build a 128-bit value, you end up with a random number with 128 bits of
/// > prediction resistance (2<sup>128</sup> * 2<sup>128</sup> =
/// > 2<sup>256</sup>). Combine two of those 128-bit values together, and you
/// > get a 256-bit number with 256 bits of prediction resistance. You can
/// > continue in this fashion to build a random value of arbitrary width and
/// > the prediction resistance will always scale with it. Because its values
/// > have multiplicative prediction resistance
/// > RDSEED is intended for seeding other PRNGs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RdSeed(());

macro_rules! imp {
    ($($t:ident, $e:ident, $instr:expr, $detect_instr:expr, $f16:ident, $f32:ident, $f64:ident;)+) => { $(
        #[cfg(target_arch = "x86")]
        unsafe fn $f64(val: &mut u64) -> i32 {
            let [a, b] = &mut *(val as *mut u64 as *mut [u32; 2]);

            match $f32(a) {
                // retry at least once
                0 => match $f32(a) {
                    0 => 0,
                    _ => $f32(b),
                },
                _ => $f32(b),
            }
        }

        impl $t {
            /// Creates a new instance or returns `None` if the `
            #[doc = $instr]
            /// ` instruction is unavailable at runtime.
            #[inline]
            pub fn new() -> Option<Self> {
                if $detect_instr {
                    Some($t(()))
                } else {
                    None
                }
            }

            /// Creates a new instance without checking if the `
            #[doc = $instr]
            /// ` instruction is available.
            #[inline]
            pub const unsafe fn new_unchecked() -> Self {
                $t(())
            }
        }

        impl TryRng for $t {
            type Error = $e;

            #[inline]
            fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), $e> {
                crate::rng::try_fill_bytes_via_next(self, buf)
            }

            #[inline]
            fn try_next_u8(&mut self) -> Result<u8, $e> {
                self.try_next_u16().map(|val| val.to_ne_bytes()[0])
            }

            #[inline]
            fn try_next_u16(&mut self) -> Result<u16, $e> {
                let mut val = 0;
                unsafe {
                    match $f16(&mut val) {
                        // retry at least once
                        0 => match $f16(&mut val) {
                            0 => Err($e(())),
                            _ => Ok(val),
                        },
                        _ => Ok(val),
                    }
                }
            }

            #[inline]
            fn try_next_u32(&mut self) -> Result<u32, $e> {
                let mut val = 0;
                unsafe {
                    match $f32(&mut val) {
                        // retry at least once
                        0 => match $f32(&mut val) {
                            0 => Err($e(())),
                            _ => Ok(val),
                        },
                        _ => Ok(val),
                    }
                }
            }

            #[inline]
            fn try_next_u64(&mut self) -> Result<u64, $e> {
                let mut val = 0;
                unsafe {
                    match $f64(&mut val) {
                        // retry at least once
                        0 => match $f64(&mut val) {
                            0 => Err($e(())),
                            _ => Ok(val),
                        },
                        _ => Ok(val),
                    }
                }
            }

            #[inline]
            fn try_next_u128(&mut self) -> Result<u128, $e> {
                let mut val = 0u128;
                let [a, b] = unsafe {
                    &mut *(&mut val as *mut u128 as *mut [u64; 2])
                };
                *a = self.try_next_u64()?;
                *b = self.try_next_u64()?;
                Ok(val)
            }
        }

        /// The error returned when the `
        #[doc = $instr]
        /// ` instruction does not generate a number.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $e(());
    )+ }
}

imp! {
    RdRand, RdRandError, "rdrand", is_x86_feature_detected!("rdrand"), _rdrand16_step, _rdrand32_step, _rdrand64_step;
    RdSeed, RdSeedError, "rdseed", is_x86_feature_detected!("rdseed"), _rdseed16_step, _rdseed32_step, _rdseed64_step;
}
