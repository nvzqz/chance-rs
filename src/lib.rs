//! [WIP] Random number generation.

#![deny(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

mod rand;
mod rand_in;
pub mod arch;
pub mod ext;
pub mod platform;
pub mod rng;

/// Commonly used traits. Meant to be imported via `use chance::prelude::*`.
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        rand::Rand,
        rand_in::RandIn,
        ext::SliceExt,
        rng::{Rng, TryRng},
    };
}

#[doc(inline)]
pub use self::{
    rand::Rand,
    rand_in::RandIn,
    rng::{Rng, TryRng},
};
