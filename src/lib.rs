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
mod con_vals;
mod dim;
mod elements;
mod err;
mod idxs;
mod model;
mod pars;
mod scalars;
mod sets;
mod slot;
mod sym_refs;
mod symbol;
mod vars;

// crate
pub(crate) use bag::{new_bag, Bag};
pub(crate) use con_vals::{ConValTyp, TConVal};
pub(crate) use dim::{Dim, DimVec};
pub(crate) use elements::Elements;
pub(crate) use err::{Res, ResValid};
pub(crate) use scalars::{Int, Num, Sca, ScaInt, ScaNum, ScaVar, Val, ValTyp, Var};
pub(crate) use sets::{Set, SetCatInt, SetCatStr, SetOrd, SetSym, SetTyp};
pub(crate) use slot::{Ptr, Slot};
pub(crate) use symbol::Symbol;
pub(crate) use vars::VarType;

#[cfg(test)]
pub(crate) use elements::ElementsIter;
#[cfg(test)]
pub(crate) use vars::Bounds;

// api
pub use model::Model;
