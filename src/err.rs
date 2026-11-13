use crate::{slot::State, Model, Set, SetTyp, Symbol};
use alloc::string::String;
use core::ops::Deref;

// res

pub type Res<T = ()> = Result<T, String>;

impl ResValid for Res {}

// TODO: fix the error messages
pub trait ResValid {
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

    fn assert_same_set_typ(a: SetTyp, b: SetTyp) -> Res {
        Self::assert(a == b, || String::from("set type mismatch"))
    }

    // panic

    fn dbg_assert_same_model<'m, 'd, T>(
        m: &'m Model<'d>,
        sym: impl Deref<Target = Symbol<'m, 'd, T>>,
    ) {
        debug_assert!(core::ptr::eq(m, sym.m), "wrong model used");
    }

    fn dbg_assert_same_set(a: SetTyp, b: SetTyp) {
        debug_assert_eq!(a, b, "wrong set type")
    }
}
