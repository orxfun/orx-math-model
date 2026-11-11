use crate::{slot::State, Model, Symbol};
use alloc::string::String;
use core::ops::Deref;

// err

#[derive(derive_new::new)]
pub struct Err {
    msg: String,
}

impl Err {
    pub fn when(condition: bool, msg: impl Fn() -> String) -> Res {
        match condition {
            true => Err(msg()),
            false => Ok(()),
        }
    }
}

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
}
