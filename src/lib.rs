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
// TODO: clean up this
#![allow(dead_code, unused_imports)]

extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

mod bag;
mod err;
mod model;
mod slot;

// crate
pub(crate) use bag::{new_bag, Bag};
pub(crate) use slot::Slot;

// api
pub use model::Model;
