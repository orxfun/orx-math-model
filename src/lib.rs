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

mod array_utils;
mod data;
mod model;
mod model_and_data;
mod model_data;
mod no_std_types;
mod raised_set;
mod symbols;

mod draft;

pub use data::{Data, DataBuilder, Number, ParData, SetData};
pub use model::Model;
pub use symbols::{par, set_of, Par, Set, SymbolRef};
