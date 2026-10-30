#![doc = include_str!("../README.md")]
#![warn(
    // missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
#![no_std]

extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

mod idx_scalars;
mod model;
mod numbers;
mod par_d1;
mod ptr;
mod sets;

pub use model::Model;
