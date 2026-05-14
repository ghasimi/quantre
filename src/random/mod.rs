//! Pseudorandom Number Generators (P-RNGs)

pub(crate) mod rng_normal;
pub(crate) mod rng_uniform;

pub use rng_normal::*;
pub use rng_uniform::*;
