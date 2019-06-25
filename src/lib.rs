//! [WIP] Random number generation.

#![deny(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

pub mod os;
pub mod rand;
pub mod rng;

#[doc(inline)]
pub use self::{
    rand::{Rand, TryRand},
    rng::{Rng, TryRng},
};
