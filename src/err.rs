use alloc::string::String;

use crate::slot::State;

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

pub trait Validation {
    fn assert(condition: bool, msg: impl FnOnce() -> String) -> Res {
        match condition {
            true => Err(msg()),
            false => Ok(()),
        }
    }

    fn assert_slot_empty(state: State) -> Res {
        Self::assert(state == State::Empty, || String::from("TODO"))
    }

    fn assert_slot_bound(state: State) -> Res {
        Self::assert(state == State::Bound, || String::from("TODO"))
    }
}
