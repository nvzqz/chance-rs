//! [WIP] Random number generation.

#![deny(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

mod rand;
pub mod arch;
pub mod platform;
pub mod rng;

#[doc(inline)]
pub use self::{
    rand::Rand,
    rng::{Rng, TryRng},
};
