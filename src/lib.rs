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

// mod arb_val;
mod bag;
mod elements;
mod err;
// mod fun;
// mod idxs;
mod con_vals;
mod idxs;
mod model;
mod pars;
mod sets;
mod slot;
mod sym_refs;
mod symbol;
mod vals;
// mod vals;

// crate
pub(crate) use bag::{new_bag, Bag};
pub(crate) use elements::{Elements, ElementsIter};
pub(crate) use err::{Res, ResValid};
// pub(crate) use fun::{FSet, FunSet};
// pub(crate) use idxs::{CatIntIdxSum, CatStrIdxSum, Idx, OrdIdxSum};
pub(crate) use con_vals::{ConValTyp, TConVal};
pub(crate) use sets::{Set, SetCatInt, SetCatStr, SetOrd, SetSym, SetTyp};
pub(crate) use slot::{Ptr, Slot};
pub(crate) use symbol::Symbol;
pub(crate) use vals::{Val, ValInt, ValNum, ValTyp};

// api
pub use model::Model;
