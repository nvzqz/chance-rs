//! [WIP] Random number generation.

#![deny(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

mod rand;
mod rand_in;
pub mod arch;
pub mod platform;
pub mod rng;

#[doc(inline)]
pub use self::{
    rand::Rand,
    rand_in::RandIn,
    rng::{Rng, TryRng},
};
