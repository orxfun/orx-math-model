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
mod con_vec;
mod constants;
mod constraints;
mod dim;
mod elements;
mod err;
mod expand;
mod free_fun;
mod idxs;
mod lin_expr;
mod model;
mod pars;
mod scalars;
mod sets;
mod slot;
mod sym_ref;
mod symbol;
mod vars;

// crate
pub(crate) use bag::Bag;
pub(crate) use con_vals::{ConValTyp, TConVal};
pub(crate) use con_vec::{ConVec, DimVec};
pub(crate) use dim::Dim;
pub(crate) use elements::{Elements, ElementsIter};
pub(crate) use err::{Res, ResValid};
pub(crate) use lin_expr::LinExpr;
pub(crate) use scalars::{AsF64, AsI64, Int, Num, ValTyp, Var};
pub(crate) use sets::{EIdx, Idx1, Idx2, Set, SetCatInt, SetCatStr, SetOrd, SetSym, SetTyp};
pub(crate) use slot::{Ptr, PtrMut, Slot};
pub(crate) use symbol::{SymWrap, Symbol};
pub(crate) use vars::{Bounds, VarD1, VarD2, VarType};

// api
pub use free_fun::{con, con1, eq, forall, geq, leq, over, sum};
pub use model::Model;
