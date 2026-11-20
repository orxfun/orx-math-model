use crate::SetTyp;
use crate::{con_vals::ConValTyp, slot::State};
use alloc::string::String;

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

    // panic

    fn dbg_assert_same_set(a: SetTyp, b: SetTyp) {
        debug_assert_eq!(a, b, "wrong set type")
    }
    fn dbg_assert_same_con_val(a: ConValTyp, b: ConValTyp) {
        debug_assert_eq!(a, b, "wrong constant value type")
    }
}
