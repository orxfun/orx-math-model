use crate::{slot::State, MetaSet, Model, SetTyp, Symbol};
use alloc::string::String;
use core::ops::Deref;

// res

pub type Res<T = ()> = Result<T, String>;

impl Validation for Res {}

// TODO: fix the error messages
pub trait Validation {
    fn assert(condition: bool, msg: impl FnOnce() -> String) -> Res {
        match condition {
            true => Ok(()),
            false => Err(msg()),
        }
    }

    fn assert_slot_empty(state: State) -> Res {
        Self::assert(state == State::Empty, || {
            String::from("slot is already bound")
        })
    }

    fn assert_slot_bound(state: State) -> Res {
        Self::assert(state == State::Bound, || String::from("slot is empty"))
    }

    fn assert_same_model<'m, 'd, T>(
        m: &'m Model<'d>,
        sym: impl Deref<Target = Symbol<'m, 'd, T>>,
    ) -> Res {
        Self::assert(core::ptr::eq(m, sym.m), || String::from("wrong model used"))
    }

    // panic

    fn dbg_assert_same_model<'m, 'd, T>(
        m: &'m Model<'d>,
        sym: impl Deref<Target = Symbol<'m, 'd, T>>,
    ) {
        debug_assert!(core::ptr::eq(m, sym.m), "wrong model used");
    }

    fn dbg_assert_same_set<S: MetaSet>(set_typ: SetTyp) {
        debug_assert_eq!(S::set_typ(), set_typ, "wrong set type")
    }
}
